use igd::aio::search_gateway;
use reqwest::get;
use scraper::{Html, Selector};
use std::error;

pub async fn upnp_external_ip_addr() -> Result<String, Box<dyn error::Error>> {
    let gateway = search_gateway(Default::default()).await?;
    let addr = gateway.get_external_ip().await?;
    Ok(addr.to_string())
}

pub async fn external_ip_addr() -> Result<String, Box<dyn error::Error>> {
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
