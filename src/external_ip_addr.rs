use anyhow::{anyhow, Context, Result};
use igd::aio::search_gateway;
use reqwest::get;
use scraper::{Html, Selector};

async fn upnp_external_ip_addr() -> Result<String> {
    let gateway = search_gateway(Default::default()).await?;
    let addr = gateway.get_external_ip().await?;
    Ok(addr.to_string())
}

async fn aws_external_ip_addr() -> Result<String> {
    let response = get("https://checkip.amazonaws.com/").await?;
    let text = response.text().await?;

    let addr_string = text.trim();
    let addr = addr_string
        .parse::<std::net::Ipv4Addr>()
        .with_context(|| format!("Invalid IPv4 address string: {}", addr_string))?;

    Ok(addr.to_string())
}

async fn dyndns_external_ip_addr() -> Result<String> {
    let response = get("http://checkip.dyndns.org/").await?;
    let text = response.text().await?;

    let document = Html::parse_document(&text);
    // `SelectorErrorKind` seems not thread safe and can't propagate.
    let body_selector =
        Selector::parse("body").map_err(|e| anyhow!("Failed to create body selector: {}", e))?;
    let body = document
        .select(&body_selector)
        .next()
        .with_context(|| format!("Invalid response: {}", text))?;

    let addr_string = body.inner_html().trim().replace("Current IP Address: ", "");
    let addr = addr_string
        .parse::<std::net::Ipv4Addr>()
        .with_context(|| format!("Invalid IPv4 address string: {}", addr_string))?;

    Ok(addr.to_string())
}

pub enum Provider {
    Upnp,
    Aws,
    Dyndns,
}

pub async fn external_ip_addr(provider: Provider) -> Result<String> {
    match provider {
        Provider::Upnp => upnp_external_ip_addr().await,
        Provider::Aws => aws_external_ip_addr().await,
        Provider::Dyndns => dyndns_external_ip_addr().await,
    }
}
