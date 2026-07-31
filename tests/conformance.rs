// Conformance: replays ../conformance/vectors.json against the memory API.
use pennypost::{Error, PennyPost, SendEmailRequest};
use serde_json::Value;

const BASE: &str = "http://127.0.0.1:8799";

fn key() -> String {
    format!("pp_live_{}", "a".repeat(48))
}

fn to_v<T: serde::Serialize>(t: T) -> Value {
    serde_json::to_value(t).unwrap()
}

fn run_op(c: &PennyPost, v: &Value, ctx: &std::collections::HashMap<String, Value>) -> Result<Value, Error> {
    let op = v["op"].as_str().unwrap();
    let p = &v["params"];
    match op {
        "sendEmail" => {
            if let Some(raw) = v.get("raw_req") {
                // invalid-shape vector: bypass the typed struct via a raw call
                let res = ureq::post(&format!("{}/v1/emails", BASE))
                    .set("Authorization", &format!("Bearer {}", key()))
                    .set("Content-Type", "application/json")
                    .send_string(&raw.to_string());
                return match res {
                    Ok(r) => Ok(serde_json::from_str(&r.into_string().unwrap()).unwrap()),
                    Err(ureq::Error::Status(status, r)) => {
                        let parsed: Value = serde_json::from_str(&r.into_string().unwrap()).unwrap_or_default();
                        let e = &parsed["error"];
                        Err(Error::Api {
                            status,
                            r#type: e["type"].as_str().unwrap_or("").into(),
                            code: e["code"].as_str().unwrap_or("").into(),
                            message: e["message"].as_str().unwrap_or("").into(),
                            param: e["param"].as_str().map(String::from),
                            retryable: e["retryable"].as_bool().unwrap_or(false),
                        })
                    }
                    Err(e) => Err(Error::Transport(Box::new(e))),
                };
            }
            let req: SendEmailRequest = serde_json::from_value(v["req"].clone()).unwrap();
            let idem = v.get("idempotency_key").and_then(|x| x.as_str());
            c.send_email(&req, idem).map(to_v)
        }
        "getEmail" => {
            let id = p["id"].as_str().map(String::from).unwrap_or_else(|| {
                ctx[v["after"].as_str().unwrap()]["accepted"][0]["id"].as_str().unwrap().to_string()
            });
            c.get_email(&id).map(to_v)
        }
        "listEmails" => {
            let pairs: Vec<(String, String)> = p.as_object().map(|o| o.iter().map(|(k, val)| (k.clone(), val_str(val))).collect()).unwrap_or_default();
            let refs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            c.list_emails(&refs).map(to_v)
        }
        "listSuppressions" => {
            let pairs: Vec<(String, String)> = p.as_object().map(|o| o.iter().map(|(k, val)| (k.clone(), val_str(val))).collect()).unwrap_or_default();
            let refs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            c.list_suppressions(&refs).map(to_v)
        }
        "addSuppression" => c.add_suppression(v["req"]["email"].as_str().unwrap()).map(to_v),
        "removeSuppression" => c.remove_suppression(p["email"].as_str().unwrap()).map(to_v),
        _ => panic!("unknown op {}", op),
    }
}

fn val_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

#[test]
fn conformance() {
    let doc: Value = serde_json::from_str(&std::fs::read_to_string("../conformance/vectors.json").unwrap()).unwrap();
    let default = PennyPost::with_base_url(&key(), BASE);
    let mut ctx = std::collections::HashMap::new();
    let mut passed = 0;
    for v in doc["vectors"].as_array().unwrap() {
        let id = v["id"].as_str().unwrap();
        let auth_client;
        let c = if let Some(a) = v.get("auth") {
            auth_client = PennyPost::with_base_url(a.as_str().unwrap(), BASE);
            &auth_client
        } else {
            &default
        };
        if let Some(exp) = v.get("expect_error") {
            match run_op(c, v, &ctx) {
                Ok(_) => panic!("{id}: expected error"),
                Err(Error::Api { status, code, param, retryable, .. }) => {
                    assert_eq!(status as u64, exp["status"].as_u64().unwrap(), "{id}: status");
                    if let Some(want) = exp["code"].as_str() {
                        assert_eq!(code, want, "{id}: code");
                    }
                    if let Some(want) = exp["param"].as_str() {
                        assert_eq!(param.as_deref(), Some(want), "{id}: param");
                    }
                    if let Some(want) = exp["retryable"].as_bool() {
                        assert_eq!(retryable, want, "{id}: retryable");
                    }
                }
                Err(e) => panic!("{id}: unexpected error kind {e}"),
            }
        } else {
            let reps = v.get("repeat").and_then(|r| r.as_u64()).unwrap_or(1);
            let mut results = Vec::new();
            for _ in 0..reps {
                results.push(run_op(c, v, &ctx).unwrap_or_else(|e| panic!("{id}: {e}")));
            }
            let r = results[0].clone();
            ctx.insert(id.to_string(), r.clone());
            let e = &v["expect"];
            let arr = |k: &str| r[k].as_array().cloned().unwrap_or_default();
            if let Some(n) = e["accepted_len"].as_u64() {
                assert_eq!(arr("accepted").len() as u64, n, "{id}: accepted");
            }
            if let Some(n) = e["suppressed_len"].as_u64() {
                assert_eq!(arr("suppressed").len() as u64, n, "{id}: suppressed");
            }
            if let Some(p) = e["id_prefix"].as_str() {
                assert!(arr("accepted")[0]["id"].as_str().unwrap().starts_with(p), "{id}: prefix");
            }
            if e["same_id_across_repeats"].as_bool() == Some(true) {
                assert_eq!(results[0]["accepted"][0]["id"], results[1]["accepted"][0]["id"], "{id}: idem");
            }
            if let Some(s) = e["subject"].as_str() {
                assert_eq!(r["subject"].as_str(), Some(s), "{id}: subject");
            }
            if e["has_events_array"].as_bool() == Some(true) {
                assert!(r["events"].is_array(), "{id}: events");
            }
            if let Some(n) = e["data_len"].as_u64() {
                assert_eq!(arr("data").len() as u64, n, "{id}: data len");
            }
            if let Some(s) = e["first_to"].as_str() {
                assert_eq!(arr("data")[0]["to"].as_str(), Some(s), "{id}: first to");
            }
            if let Some(s) = e["reason"].as_str() {
                assert_eq!(r["reason"].as_str(), Some(s), "{id}: reason");
            }
            if let Some(s) = e["suppressed_reason"].as_str() {
                assert_eq!(arr("suppressed")[0]["reason"].as_str(), Some(s), "{id}: sup reason");
            }
            if let Some(want) = e["contains_email"].as_str() {
                assert!(arr("data").iter().any(|s| s["email"].as_str() == Some(want)), "{id}: contains");
            }
            if let Some(b) = e["removed"].as_bool() {
                assert_eq!(r["removed"].as_bool(), Some(b), "{id}: removed");
            }
        }
        passed += 1;
        println!("  ok {id}");
    }
    println!("\nrust conformance: {passed}/12");
}
