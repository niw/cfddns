Cloudflare Dynamic DNS Client
=============================

A simple command line tool to update Cloudflare DNS record with
external IP address to use it as dynamic DNS.

NAME
----

`cfddns` -- A simple command line tool to update Cloudflare DNS record.

SYNOPSIS
--------

```
cfddns --api-token <API_TOKEN> --zone-id <ZONE_ID> COMMAND [OPTIONS]
```

- `--api-token` or `API_TOKEN` environment variable is a token that is
  created at <https://dash.cloudflare.com/profile/api-tokens>.

- `--zone-id` or `ZONE_ID` enrivonment variable is found on "Overview"
  page on each domain name on Cloudflare dashboard.

### Command

`update` -- Update a Cloudflare DNS record with an external IP address.

#### Options

- `--record-id` to specify the record id to update. Use `list` command
  to find this.

- `--record-type` is DNS record type. Default to use `A`.

- `--name` is DNS record name.

- `--provider` is the provider to fetch the external IP address.
  Possible values is either
  `upnp` to use UPnP, `aws` to use `https://checkip.amazonaws.com/`,
  `dyndns` to use `http://checkip.dyndns.org/`.
  Default to use `upnp`.

`list` -- List all Cloudflare DNS records.

#### Options

This command takes no options.

`print-launchd-plist` -- Print `launchd(8)` plist file to update DNS record. Useful to install `cfddns` as a repeated task on macOS.

#### Options

This command takes same options as `update` command.

DESCRIPTION
-----------

`cfddns` is a simple command line tool to update Cloudflare DNS record
with external IP address to use it as dynamic DNS.
External IP address is fetched by UPnP locally or 3rd party API.


USAGE
-----

Use [Cargo](https://www.rust-lang.org/) to build it.

```bash
cargo build --release # Build binary.
target/release/cfddns help # Print usage.
```
