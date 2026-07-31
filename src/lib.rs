// Generated from api/openapi.json (sha 7c58f5917c4b) by sdk/generate.ts — do not edit by hand.
// PennyPost: the affordable email API. https://pennypost.io/docs

pub const SPEC_SHA: &str = "7c58f5917c4b";

const DEFAULT_BASE: &str = "https://api.pennypost.io";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveResult {
    pub removed: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pennypost: {message} ({code})")]
    Api { status: u16, r#type: String, code: String, message: String, param: Option<String>, retryable: bool },
    #[error(transparent)]
    Transport(#[from] Box<ureq::Error>),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Recipient {
    pub to: String,
    pub id: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendEmailResponse {
    pub accepted: Vec<Recipient>,
    pub suppressed: Vec<Recipient>,
    pub quarantined: Option<Vec<Recipient>>,
    pub failed: Option<Vec<Recipient>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailEvent {
    pub r#type: String,
    pub code: Option<String>,
    pub reason: Option<String>,
    pub at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub id: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub status: String,
    pub mode: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub created_at: String,
    pub events: Option<Vec<EmailEvent>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailPage {
    pub data: Vec<Email>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Suppression {
    pub email: String,
    pub reason: String,
    pub source_email_id: Option<String>,
    pub at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuppressionPage {
    pub data: Vec<Suppression>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

pub struct PennyPost {
    key: String,
    base: String,
}

impl PennyPost {
    pub fn new(api_key: &str) -> Self {
        Self { key: api_key.to_string(), base: DEFAULT_BASE.to_string() }
    }

    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        Self { key: api_key.to_string(), base: base_url.trim_end_matches('/').to_string() }
    }

    fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<serde_json::Value>,
        query: &[(&str, &str)],
        idem: Option<&str>,
    ) -> Result<T, Error> {
        let mut url = format!("{}{}", self.base, path);
        let qs: Vec<String> = query
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect();
        if !qs.is_empty() {
            url = format!("{}?{}", url, qs.join("&"));
        }
        let mut req = ureq::request(method, &url).set("Authorization", &format!("Bearer {}", self.key));
        if let Some(k) = idem {
            req = req.set("Idempotency-Key", k);
        }
        let res = match body {
            Some(b) => req.set("Content-Type", "application/json").send_string(&b.to_string()),
            None => req.call(),
        };
        match res {
            Ok(r) => Ok(serde_json::from_str(&r.into_string()?)?),
            Err(ureq::Error::Status(status, r)) => {
                let parsed: serde_json::Value = serde_json::from_str(&r.into_string()?).unwrap_or_default();
                let e = &parsed["error"];
                Err(Error::Api {
                    status,
                    r#type: e["type"].as_str().unwrap_or("provider").to_string(),
                    code: e["code"].as_str().unwrap_or("unknown_error").to_string(),
                    message: e["message"].as_str().unwrap_or("request failed").to_string(),
                    param: e["param"].as_str().map(String::from),
                    retryable: e["retryable"].as_bool().unwrap_or(status >= 500),
                })
            }
            Err(e) => Err(Error::Transport(Box::new(e))),
        }
    }

    /// POST /v1/emails
    pub fn send_email(&self, req: &SendEmailRequest, idempotency_key: Option<&str>) -> Result<SendEmailResponse, Error> {
        self.request("POST", "/v1/emails", Some(serde_json::to_value(req)?), &[], idempotency_key)
    }

    /// GET /v1/emails
    pub fn list_emails(&self, params: &[(&str, &str)]) -> Result<EmailPage, Error> {
        self.request("GET", "/v1/emails", None, params, None)
    }

    /// GET /v1/emails/{id}
    pub fn get_email(&self, id: &str) -> Result<Email, Error> {
        self.request("GET", &format!("/v1/emails/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// GET /v1/suppressions
    pub fn list_suppressions(&self, params: &[(&str, &str)]) -> Result<SuppressionPage, Error> {
        self.request("GET", "/v1/suppressions", None, params, None)
    }

    /// POST /v1/suppressions
    pub fn add_suppression(&self, email: &str) -> Result<Suppression, Error> {
        self.request("POST", "/v1/suppressions", Some(serde_json::json!({"email": email})), &[], None)
    }

    /// DELETE /v1/suppressions/{email}
    pub fn remove_suppression(&self, email: &str) -> Result<RemoveResult, Error> {
        self.request("DELETE", &format!("/v1/suppressions/{}", urlencoding::encode(email)), None, &[], None)
    }
}
