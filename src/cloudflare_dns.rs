use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
struct DnsRecord {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: i32,
}

#[derive(Deserialize)]
struct UpdateDnsRecordResponse {
    success: bool,
}

pub async fn update_record(
    api_token: &str,
    zone_id: &str,
    record_id: &str,
    record_type: &str,
    name: &str,
    ip: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, record_id
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let dns_record = DnsRecord {
        record_type: record_type.to_string(),
        name: name.to_string(),
        content: ip.to_string(),
        ttl: 1,
    };

    let response = client
        .put(url)
        .headers(headers)
        .json(&dns_record)
        .send()
        .await?;

    let update_response: UpdateDnsRecordResponse = response.json().await?;

    if update_response.success {
        Ok(())
    } else {
        Err("Failed to update DNS record".into())
    }
}
