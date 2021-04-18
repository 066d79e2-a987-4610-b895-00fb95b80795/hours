use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::report::Report;

const API_ROOT: &'static str = "https://api.github.com";

pub struct GistClient {
    api_key: String,
    gist_id: String,
    client: Client,
}

pub struct GetReport {
    pub report: Report,
    pub last_updated: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize)]
struct GistModel {
    updated_at: String,
    files: HashMap<String, GistFileModel>,
}

#[derive(Debug, Deserialize)]
struct GistFileModel {
    content: String,
}

impl GistClient {
    pub fn new(api_key: String, gist_id: String) -> Self {
        let client = Client::new();
        Self {
            client,
            gist_id,
            api_key,
        }
    }

    pub async fn update(&self, report: &Report) {
        self.client
            .patch(format!("{}/gists/{}", API_ROOT, self.gist_id))
            .json(&json!({
                "files": {
                    "hours": {
                        "content": report.0
                    }
                }
            }))
            .header("User-Agent", "whatever")
            .header("Authorization", format!("Token {}", self.api_key))
            .send()
            .await
            .unwrap();
    }

    pub async fn get(&self) -> GetReport {
        let res = self
            .client
            .get(format!("{}/gists/{}", API_ROOT, self.gist_id))
            .header("User-Agent", "whatever")
            .header("Authorization", format!("Token {}", self.api_key))
            .send()
            .await
            .unwrap();
        let gist: GistModel = res.json().await.unwrap();
        GetReport {
            report: Report(gist.files["hours"].content.clone()),
            last_updated: DateTime::parse_from_rfc3339(&gist.updated_at).unwrap(),
        }
    }
}
