use clap::{Args, Parser, Subcommand, ValueEnum};
use std::process::exit;
use std::fmt;

mod cloudflare_dns;
mod external_ip_addr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// API token for Cloudflare
    #[arg(long, env)]
    api_token: String,

    /// Zone ID for Cloudflare
    #[arg(long, env)]
    zone_id: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Provider {
    /// Use UPnP
    Upnp,
    /// Use `https://checkip.amazonaws.com/`
    Aws,
    /// Use `http://checkip.dyndns.org/`
    Dyndns,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Provider::Upnp => write!(f, "upnp"),
            Provider::Aws => write!(f, "aws"),
            Provider::Dyndns => write!(f, "Dyndns"),
        }
    }
}

#[derive(Args)]
struct UpdateArgs {
    /// Record ID to update
    #[arg(long)]
    record_id: String,

    /// DNS record type
    #[arg(long, default_value = "A")]
    record_type: String,

    /// DNS record name
    #[arg(long)]
    name: String,

    /// Provider to fetch the external IP address
    #[arg(long, default_value = "upnp")]
    provider: Provider,
}

#[derive(Subcommand)]
enum Commands {
    /// Update a DNS record
    Update(UpdateArgs),
    /// List all DNS records
    List,
    /// Print a launchd plist to update DNS record
    PrintUpdateLaunchdPlist(UpdateArgs),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Update(update_args) => {
            let provider = match update_args.provider {
                Provider::Upnp => external_ip_addr::Provider::Upnp,
                Provider::Aws => external_ip_addr::Provider::Aws,
                Provider::Dyndns => external_ip_addr::Provider::Dyndns,
            };
            let ip_addr = match external_ip_addr::external_ip_addr(provider).await {
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

        Commands::PrintUpdateLaunchdPlist(update_args) => {
            let current_exe_path = match std::env::current_exe() {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Failed to get current executable path: {}", e);
                    exit(1);
                }
            };

            println!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>at.niw.cfddns</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>API_TOKEN</key>
        <string>{}</string>
        <key>ZONE_ID</key>
        <string>{}</string>
    </dict>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>update</string>
        <string>--record-id</string>
        <string>{}</string>
        <string>--record-type</string>
        <string>{}</string>
        <string>--name</string>
        <string>{}</string>
        <string>--provider</string>
        <string>{}</string>
    </array>
    <key>StartInterval</key>
    <integer>3600</integer>
</dict>
</plist>"#,
                args.api_token,
                args.zone_id,
                current_exe_path.to_string_lossy(),
                update_args.record_id,
                update_args.record_type,
                update_args.name,
                update_args.provider
            );
        }
    }
}
