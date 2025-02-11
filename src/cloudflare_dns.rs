use anyhow::{ensure, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};

fn authorized_client(api_token: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_token))?,
    );

    let client = Client::builder().default_headers(headers).build()?;

    Ok(client)
}

#[derive(Serialize)]
struct UpdateRecord {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: i32,
}

#[derive(Deserialize)]
struct UpdateRecordResponse {
    success: bool,
}

pub async fn update_record(
    api_token: &str,
    zone_id: &str,
    record_id: &str,
    record_type: &str,
    name: &str,
    ip: &str,
) -> Result<()> {
    let client = authorized_client(api_token)?;
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, record_id
    );
    let update_record = UpdateRecord {
        record_type: record_type.to_string(),
        name: name.to_string(),
        content: ip.to_string(),
        ttl: 1,
    };

    let response = client.put(url).json(&update_record).send().await?;

    let update_response: UpdateRecordResponse = response.json().await?;
    ensure!(update_response.success, "Failed to update DNS record");

    Ok(())
}

#[derive(Deserialize)]
pub struct Record {
    pub id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub ttl: i32,
}

#[derive(Deserialize)]
struct RecordsResponse {
    success: bool,
    result: Option<Vec<Record>>,
}

pub async fn list_records(api_token: &str, zone_id: &str) -> Result<Vec<Record>> {
    let client = authorized_client(api_token)?;
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
        zone_id
    );

    let response = client.get(url).send().await?;

    let records_response: RecordsResponse = response.json().await?;
    ensure!(records_response.success, "Failed to list DNS records");

    Ok(records_response.result.unwrap_or_default())
}
