use igd::aio::search_gateway;
use reqwest::get;
use scraper::{Html, Selector};
use std::error;

async fn upnp() -> Result<String, Box<dyn error::Error>> {
    let gateway = search_gateway(Default::default()).await?;
    let addr = gateway.get_external_ip().await?;
    Ok(addr.to_string())
}

async fn aws() -> Result<String, Box<dyn error::Error>> {
    let response = get("https://checkip.amazonaws.com/").await?;
    let text = response.text().await?;
    if let Ok(addr) = text.parse::<std::net::Ipv4Addr>() {
        Ok(addr.to_string())
    } else {
        Err(format!("Invalid IPv4 address string: {}", text).into())
    }
}

async fn dyndns() -> Result<String, Box<dyn error::Error>> {
    let response = get("http://checkip.dyndns.org/").await?;
    let text = response.text().await?;

    let document = Html::parse_document(&text);
    let body_selector = Selector::parse("body")?;
    if let Some(body) = document.select(&body_selector).next() {
        let addr_string = body.inner_html().trim().replace("Current IP Address: ", "");
        if let Ok(addr) = addr_string.parse::<std::net::Ipv4Addr>() {
            Ok(addr.to_string())
        } else {
            Err(format!("Invalid IPv4 address string: {}", addr_string).into())
        }
    } else {
        Err(format!("Invalid response: {}", text).into())
    }
}

pub async fn external_ip_addr(provider: &str) -> Result<String, Box<dyn error::Error>> {
    match provider {
        "upnp" => upnp().await,
        "aws" => aws().await,
        "dyndns" => dyndns().await,
        _ => Err(format!("Invalid provider: {}", provider).into()),
    }
}
