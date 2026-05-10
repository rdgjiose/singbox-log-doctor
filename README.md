# singbox-log-doctor

A small Rust command-line tool for analysing sing-box logs in a home network environment.

## Why I built this

I use sing-box with TUN, Tailscale, AdGuardHome, Docker and Cloudflare Tunnel on a small Armbian server. During troubleshooting, I often need to identify repeated errors such as DNS packet errors, Tailscale endpoint timeouts, and unexpected routing decisions.

This tool helps summarise these logs quickly.

## Current features

- Count total log lines
- Count ERROR and WARN messages
- Detect DNS packet errors:
  - `bad rdata`
  - `buffer size too small`
- Detect Tailscale / ts-out issues:
  - `i/o timeout`
  - `connection timed out`
  - `endpoint not connected`
- Detect possible Google IP traffic routed through `ts-out`
- Print simple troubleshooting advice

## Usage

Run the tool with a sample log file:

```bash
cargo run -- examples/sample-singbox.log
```
Run with stdin:

```bash
cat examples/sample-singbox.log | cargo run
```
On a server, it can be used with Docker logs:

```bash
docker logs sing-box --since 10m | singbox-log-doctor
```


#Example output:
singbox-log-doctor report
=========================
Total lines: 6
Errors: 5
Warnings: 1

DNS issues:
  bad rdata: 1
  buffer size too small: 1

Tailscale / ts-out issues:
  timeout: 2
  endpoint not connected: 1

Routing issues:
  Google IP through ts-out: 1

#Roadmap

* Support reading logs from stdin
* Support JSON output
* Add more routing issue detection rules
* Add unit tests
* Build release binaries
