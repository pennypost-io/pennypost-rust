// Generated from api/openapi.json (sha 13bb405e5ef5) by sdk/generate.ts — do not edit by hand.
// PennyPost: the affordable email API. https://pennypost.io/docs

pub const SPEC_SHA: &str = "13bb405e5ef5";

const DEFAULT_BASE: &str = "https://api.pennypost.io";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendEmailResponse {
    pub accepted: Vec<Recipient>,
    pub suppressed: Vec<Recipient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantined: Option<Vec<Recipient>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<Recipient>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailEvent {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EmailEvent>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailPage {
    pub data: Vec<Email>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Suppression {
    pub email: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_email_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuppressionPage {
    pub data: Vec<Suppression>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookEndpoint {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive_failures: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_success_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookList {
    pub data: Vec<WebhookEndpoint>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookTestResult {
    pub delivered: bool,
    pub endpoint_status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    pub plan: String,
    pub status: String,
    pub daily_cap: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_to_date_sent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_on_file: Option<bool>,
    pub enforcement: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_send_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_live_send_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiKeySummary {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub prefix: String,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiKeyList {
    pub data: Vec<ApiKeySummary>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Audience {
    pub id: String,
    pub name: String,
    pub contact_count: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudienceList {
    pub data: Vec<Audience>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContactInput {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contact {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContactPage {
    pub data: Vec<Contact>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateAudienceRequest {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddContactsRequest {
    pub contacts: Vec<ContactInput>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddContactsResult {
    pub added: i64,
    pub duplicates: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateContactRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BroadcastCounters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounced: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complained: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Broadcast {
    pub id: String,
    pub audience_id: String,
    pub from: String,
    pub subject: String,
    pub status: String,
    pub counters: BroadcastCounters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<BroadcastFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BroadcastList {
    pub data: Vec<Broadcast>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateBroadcastRequest {
    pub audience_id: String,
    pub from: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<BroadcastFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateBroadcastRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<BroadcastFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendBroadcastRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_opt_in: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendEmailBatchResponse {
    pub data: Vec<SendEmailResponse>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BroadcastFilter {
    pub property: String,
    pub equals: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestBroadcastRequest {
    pub to: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestBroadcastResult {
    pub sent: i64,
    pub to: Vec<String>,
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

    /// GET /v1/audiences
    pub fn list_audiences(&self) -> Result<AudienceList, Error> {
        self.request("GET", "/v1/audiences", None, &[], None)
    }

    /// POST /v1/audiences
    pub fn create_audience(&self, req: &CreateAudienceRequest) -> Result<Audience, Error> {
        self.request("POST", "/v1/audiences", Some(serde_json::to_value(req)?), &[], None)
    }

    /// GET /v1/audiences/{id}
    pub fn get_audience(&self, id: &str) -> Result<Audience, Error> {
        self.request("GET", &format!("/v1/audiences/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// DELETE /v1/audiences/{id}
    pub fn delete_audience(&self, id: &str) -> Result<RemoveResult, Error> {
        self.request("DELETE", &format!("/v1/audiences/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// GET /v1/audiences/{id}/contacts
    pub fn list_audience_contacts(&self, id: &str, params: &[(&str, &str)]) -> Result<ContactPage, Error> {
        self.request("GET", &format!("/v1/audiences/{}/contacts", urlencoding::encode(id)), None, params, None)
    }

    /// POST /v1/audiences/{id}/contacts
    pub fn add_audience_contacts(&self, id: &str, req: &AddContactsRequest) -> Result<AddContactsResult, Error> {
        self.request("POST", &format!("/v1/audiences/{}/contacts", urlencoding::encode(id)), Some(serde_json::to_value(req)?), &[], None)
    }

    /// DELETE /v1/audiences/{id}/contacts/{email}
    pub fn delete_audience_contact(&self, id: &str, email: &str) -> Result<RemoveResult, Error> {
        self.request("DELETE", &format!("/v1/audiences/{}/contacts/{}", urlencoding::encode(id), urlencoding::encode(email)), None, &[], None)
    }

    /// PATCH /v1/audiences/{id}/contacts/{email}
    pub fn update_audience_contact(&self, id: &str, email: &str, req: &UpdateContactRequest) -> Result<Contact, Error> {
        self.request("PATCH", &format!("/v1/audiences/{}/contacts/{}", urlencoding::encode(id), urlencoding::encode(email)), Some(serde_json::to_value(req)?), &[], None)
    }

    /// GET /v1/broadcasts
    pub fn list_broadcasts(&self) -> Result<BroadcastList, Error> {
        self.request("GET", "/v1/broadcasts", None, &[], None)
    }

    /// POST /v1/broadcasts
    pub fn create_broadcast(&self, req: &CreateBroadcastRequest) -> Result<Broadcast, Error> {
        self.request("POST", "/v1/broadcasts", Some(serde_json::to_value(req)?), &[], None)
    }

    /// GET /v1/broadcasts/{id}
    pub fn get_broadcast(&self, id: &str) -> Result<Broadcast, Error> {
        self.request("GET", &format!("/v1/broadcasts/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// PATCH /v1/broadcasts/{id}
    pub fn update_broadcast(&self, id: &str, req: &UpdateBroadcastRequest) -> Result<Broadcast, Error> {
        self.request("PATCH", &format!("/v1/broadcasts/{}", urlencoding::encode(id)), Some(serde_json::to_value(req)?), &[], None)
    }

    /// DELETE /v1/broadcasts/{id}
    pub fn delete_broadcast(&self, id: &str) -> Result<RemoveResult, Error> {
        self.request("DELETE", &format!("/v1/broadcasts/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// POST /v1/broadcasts/{id}/send
    pub fn send_broadcast(&self, id: &str, req: &SendBroadcastRequest) -> Result<Broadcast, Error> {
        self.request("POST", &format!("/v1/broadcasts/{}/send", urlencoding::encode(id)), Some(serde_json::to_value(req)?), &[], None)
    }

    /// POST /v1/broadcasts/{id}/cancel
    pub fn cancel_broadcast(&self, id: &str) -> Result<Broadcast, Error> {
        self.request("POST", &format!("/v1/broadcasts/{}/cancel", urlencoding::encode(id)), None, &[], None)
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

    /// GET /v1/webhooks
    pub fn list_webhooks(&self) -> Result<WebhookList, Error> {
        self.request("GET", "/v1/webhooks", None, &[], None)
    }

    /// POST /v1/webhooks
    pub fn create_webhook(&self, req: &CreateWebhookRequest) -> Result<WebhookEndpoint, Error> {
        self.request("POST", "/v1/webhooks", Some(serde_json::to_value(req)?), &[], None)
    }

    /// DELETE /v1/webhooks/{id}
    pub fn delete_webhook(&self, id: &str) -> Result<RemoveResult, Error> {
        self.request("DELETE", &format!("/v1/webhooks/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// POST /v1/webhooks/{id}/test
    pub fn test_webhook(&self, id: &str) -> Result<WebhookTestResult, Error> {
        self.request("POST", &format!("/v1/webhooks/{}/test", urlencoding::encode(id)), None, &[], None)
    }

    /// GET /v1/account
    pub fn get_account(&self) -> Result<Account, Error> {
        self.request("GET", "/v1/account", None, &[], None)
    }

    /// GET /v1/keys
    pub fn list_keys(&self) -> Result<ApiKeyList, Error> {
        self.request("GET", "/v1/keys", None, &[], None)
    }

    /// POST /v1/keys
    pub fn create_key(&self, req: &CreateKeyRequest) -> Result<ApiKeySummary, Error> {
        self.request("POST", "/v1/keys", Some(serde_json::to_value(req)?), &[], None)
    }

    /// DELETE /v1/keys/{id}
    pub fn revoke_key(&self, id: &str) -> Result<RemoveResult, Error> {
        self.request("DELETE", &format!("/v1/keys/{}", urlencoding::encode(id)), None, &[], None)
    }

    /// POST /v1/emails/batch
    pub fn send_email_batch(&self, req: &[SendEmailRequest], idempotency_key: Option<&str>) -> Result<SendEmailBatchResponse, Error> {
        self.request("POST", "/v1/emails/batch", Some(serde_json::to_value(req)?), &[], idempotency_key)
    }

    /// POST /v1/broadcasts/{id}/test
    pub fn test_broadcast(&self, id: &str, req: &TestBroadcastRequest) -> Result<TestBroadcastResult, Error> {
        self.request("POST", &format!("/v1/broadcasts/{}/test", urlencoding::encode(id)), Some(serde_json::to_value(req)?), &[], None)
    }
}
