use clap::Parser;
use std::process::exit;

mod cloudflare_dns;
mod external_ip_addr;

#[derive(Parser, Debug)]
#[command(
    name = "cfddns",
    about = "Update Cloudflare DNS record with external IP address.",
    version = "0.1.0"
)]
struct Args {
    #[arg(long, env)]
    api_token: String,

    #[arg(long, env)]
    zone_id: String,

    #[arg(long)]
    record_id: String,

    #[arg(long, default_value = "A")]
    record_type: String,

    #[arg(long)]
    name: String,

    #[arg(long)]
    use_upnp: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let ip_addr = match if args.use_upnp {
        external_ip_addr::upnp_external_ip_addr().await
    } else {
        external_ip_addr::external_ip_addr().await
    } {
        Ok(ip_addr) => ip_addr,
        Err(e) => {
            eprintln!("Failed to get external IP address: {}", e);
            exit(1);
        }
    };

    match cloudflare_dns::update_record(
        &args.api_token,
        &args.zone_id,
        &args.record_id,
        &args.record_type,
        &args.name,
        &ip_addr,
    )
    .await
    {
        Ok(_) => {
            println!("Updated DNS record: {} IP address: {}", args.name, ip_addr);
        }
        Err(e) => {
            eprintln!("Failed to update DNS record: {}", e);
            exit(1);
        }
    }
}
