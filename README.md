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
cfddns [OPTIONS] --api-token <API_TOKEN> --zone-id <ZONE_ID> --record-id <RECORD_ID> --name <NAME>
```

### Options

- `--api-token` or `API_TOKEN` environment variable is a token that is
  created at <https://dash.cloudflare.com/profile/api-tokens>.

- `--zone-id` or `ZONE_ID` enrivonment variable is found on "Overview"
  page on each domain name on Cloudflare dashboard.

- `--record-id` is found by making [List DNS Records API](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-list-dns-records) call.
  Use `script/list_dns_records.sh` for example. See USAGE as well.

- `--record-type` is DNS record type. Default to use `A`.

- `--name` is DNS record name.

- `--use-upnp` to use UPnP to get external IP address. Default to use
  `http://checkip.dyndns.org` response.

DESCRIPTION
-----------

`cfddns` is a simple command line tool to update Cloudflare DNS record
with external IP address to use it as dynamic DNS.
External IP address is fetched by 3rd party API or UPnP.


USAGE
-----

Use [Cargo](https://www.rust-lang.org/) to build it.

```bash
cargo build --release
target/release/cfddns
```

To find `RECORD_ID`, `script/list_dns_records.sh` may help.

```bash
API_KEY=... ZONE_ID=... script/list_dns_records.sh|jq '.result[]|{id: .id, name: .name}'
```
