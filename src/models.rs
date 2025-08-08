use serde::*;

//Documentation is: https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAddress {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicTemplateData {
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
}
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Personalization {
    pub to: Vec<EmailAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<EmailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<EmailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_template_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub content: String,
    pub filename: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub disposition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asm {
    pub group_id: i32,
    pub groups_to_display: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BypassListManagement {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Footer {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxMode {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailSettings {
    pub bypass_list_management: BypassListManagement,
    pub footer: Footer,
    pub sandbox_mode: SandboxMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClickTracking {
    pub enable: bool,
    pub enable_text: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTracking {
    pub enable: bool,
    pub substitution_tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionTracking {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingSettings {
    pub click_tracking: ClickTracking,
    pub open_tracking: OpenTracking,
    pub subscription_tracking: SubscriptionTracking,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailContentJsonModel {
    #[serde(rename = "type")]
    pub content_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailPostModel {
    pub personalizations: Vec<Personalization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<EmailAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EmailAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EmailAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<EmailContentJsonModel>>,

    pub send_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm: Option<Asm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_settings: Option<MailSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_settings: Option<TrackingSettings>,
}
