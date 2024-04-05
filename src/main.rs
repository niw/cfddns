use clap::{Args, Parser, Subcommand};
use std::process::exit;

mod cloudflare_dns;
mod external_ip_addr;

#[derive(Parser)]
#[command(
    name = "cfddns",
    about = "Update Cloudflare DNS record with external IP address.",
    version = "0.1.0"
)]
struct Cli {
    #[arg(long, env)]
    api_token: String,

    #[arg(long, env)]
    zone_id: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct UpdateArgs {
    #[arg(long)]
    record_id: String,

    #[arg(long, default_value = "A")]
    record_type: String,

    #[arg(long)]
    name: String,

    #[arg(long, default_value = "upnp")]
    provider: String,
}

#[derive(Subcommand)]
enum Commands {
    Update(UpdateArgs),
    List,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Update(update_args) => {
            let ip_addr = match external_ip_addr::external_ip_addr(&update_args.provider).await {
                Ok(ip_addr) => ip_addr,
                Err(e) => {
                    eprintln!("Failed to get external IP address: {}", e);
                    exit(1);
                }
            };

            match cloudflare_dns::update_record(
                &args.api_token,
                &args.zone_id,
                &update_args.record_id,
                &update_args.record_type,
                &update_args.name,
                &ip_addr,
            )
            .await
            {
                Ok(_) => {
                    println!(
                        "Updated DNS record: {} IP address: {}",
                        update_args.name, ip_addr
                    );
                }
                Err(e) => {
                    eprintln!("Failed to update DNS record: {}", e);
                    exit(1);
                }
            }
        }

        Commands::List => {
            let records = match cloudflare_dns::list_records(&args.api_token, &args.zone_id).await {
                Ok(records) => records,
                Err(e) => {
                    eprintln!("Failed to list DNS records: {}", e);
                    exit(1);
                }
            };

            for record in records {
                println!(
                    "Record ID: {} type: {} name: {} content: {}",
                    record.id, record.record_type, record.name, record.content
                );
            }
        }
    }
}
