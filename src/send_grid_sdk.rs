use flurl::FlUrl;
use rust_extensions::{StrOrString, base64::IntoBase64};

use super::models::*;

//const EU_API_URL: &'static str = "https://api.eu.sendgrid.com";
const GLOBAL_API_URL: &'static str = "https://api.sendgrid.com";

pub enum AttachmentDisposition {
    Attachment,
    Inline,
}

impl AttachmentDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            AttachmentDisposition::Attachment => "attachment",
            AttachmentDisposition::Inline => "inline",
        }
    }
}

pub struct SendGrid {
    api_key: StrOrString<'static>,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: Option<String>,
    pub template_id: Option<String>,
    pub dynamic_template_data: Option<serde_json::Value>,
    pub text_content: Vec<String>,
    pub html_content: Vec<String>,

    pub from: Option<EmailAddress>,
    attachments: Vec<EmailAttachment>,
}

impl SendGrid {
    pub fn new(api_key: impl Into<StrOrString<'static>>) -> Self {
        Self {
            api_key: api_key.into(),
            to: vec![],
            subject: Default::default(),
            text_content: Default::default(),
            html_content: Default::default(),
            template_id: None,
            from: None,
            attachments: Default::default(),
            dynamic_template_data: None,
            cc: vec![],
            bcc: vec![],
        }
    }

    pub fn set_to(
        mut self,
        email: impl Into<StrOrString<'static>>,
        name: Option<StrOrString<'static>>,
    ) -> Self {
        let email: StrOrString<'static> = email.into();

        self.to.push(EmailAddress {
            email: email.to_string(),
            name: name.map(|itm| itm.to_string()),
        });
        self
    }

    pub fn set_from(
        mut self,
        email: impl Into<StrOrString<'static>>,
        name: Option<StrOrString<'static>>,
    ) -> Self {
        let email: StrOrString<'static> = email.into();

        self.from = Some(EmailAddress {
            email: email.to_string(),
            name: name.map(|itm| itm.to_string()),
        });
        self
    }

    pub fn set_subject(mut self, subject: impl Into<StrOrString<'static>>) -> Self {
        self.subject = Some(subject.into().to_string());
        self
    }

    pub fn set_text_content(mut self, text_content: impl Into<StrOrString<'static>>) -> Self {
        let text_content = text_content.into();
        self.text_content.push(text_content.to_string());
        self
    }

    pub fn set_html_content(mut self, html_content: impl Into<StrOrString<'static>>) -> Self {
        let html_content = html_content.into();
        self.html_content.push(html_content.to_string());
        self
    }

    pub fn add_attachment(
        mut self,
        filename: impl Into<StrOrString<'static>>,
        content_type: impl Into<StrOrString<'static>>,
        disposition: AttachmentDisposition,
        content: Vec<u8>,
    ) -> Self {
        let file_name = filename.into();
        let content_type = content_type.into();
        self.attachments.push(EmailAttachment {
            content: content.into_base64(),
            filename: file_name.to_string(),
            content_type: content_type.to_string(),
            disposition: disposition.as_str().into(),
        });
        self
    }

    pub fn set_template_id(
        mut self,
        template_id: String,
        dynamic_template_data: serde_json::Value,
    ) -> Self {
        self.template_id = Some(template_id);
        self.dynamic_template_data = Some(dynamic_template_data);
        self
    }

    pub fn set_cc(mut self, email: impl Into<StrOrString<'static>>) -> Self {
        let email: StrOrString<'static> = email.into();
        self.cc.push(EmailAddress {
            email: email.to_string(),
            name: None,
        });
        self
    }

    pub fn set_bcc(mut self, email: impl Into<StrOrString<'static>>) -> Self {
        let email: StrOrString<'static> = email.into();
        self.bcc.push(EmailAddress {
            email: email.to_string(),
            name: None,
        });
        self
    }

    /*
       pub fn set_template_id(mut self, template_id: impl Into<StrOrString<'static>>) -> Self {
           let template_id = template_id.into();
           self.template_id = Some(template_id.to_string());
           self
       }
    */
    pub async fn send(mut self) {
        let from = self.from.take();
        let mut json_model = EmailPostModel {
            personalizations: vec![Personalization {
                to: self.to,
                dynamic_template_data: self.dynamic_template_data.take(),
                cc: None,
                bcc: None,
            }],
            content: None,
            from: from.clone(),
            reply_to: from,
            attachments: None,
            template_id: self.template_id.take(),
            categories: None,
            subject: self.subject,
            send_at: None,
            batch_id: None,
            asm: None,
            ip_pool_name: None,
            mail_settings: None,
            tracking_settings: None,
        };

        let mut content = vec![];

        for text_content in self.text_content {
            content.push(EmailContentJsonModel {
                content_type: "text/plain".into(),
                value: text_content,
            });
        }

        let mut cc = vec![];

        for itm in self.cc.drain(..) {
            cc.push(itm);
        }

        if cc.len() > 0 {
            json_model.personalizations[0].cc = Some(cc);
        }

        let mut bcc = vec![];

        for itm in self.bcc.drain(..) {
            bcc.push(itm);
        }

        if bcc.len() > 0 {
            json_model.personalizations[0].bcc = Some(bcc);
        }

        for html_content in self.html_content {
            content.push(EmailContentJsonModel {
                content_type: "text/html".into(),
                value: html_content,
            });
        }

        if content.len() > 0 {
            json_model.content = Some(content);
        }

        if self.attachments.len() > 0 {
            json_model.attachments = Some(self.attachments);
        }

        println!("{:#?}", json_model);

        let mut response = FlUrl::new(GLOBAL_API_URL)
            .append_path_segment("v3")
            .append_path_segment("mail")
            .append_path_segment("send")
            .with_header("Authorization", format!("Bearer {}", self.api_key.as_str()))
            .post(&json_model)
            .await
            .unwrap();

        let status_code = response.get_status_code();

        println!("send_grid StatusCode: {}", response.get_status_code());

        if status_code < 200 || status_code > 204 {
            let body = response.get_body_as_str().await.unwrap();
            println!("send_grid Body: {}", body);
        }
    }
}
