use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let content = if args.len() >= 2 {
        let file_path = &args[1];

        match fs::read_to_string(file_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Failed to read file '{}': {}", file_path, error);
                return;
            }
        }
    } else {
        let mut input = String::new();

        if let Err(error) = io::stdin().read_to_string(&mut input) {
            eprintln!("Failed to read from stdin: {}", error);
            return;
        }

        if input.trim().is_empty() {
            eprintln!("Usage:");
            eprintln!("  singbox-log-doctor <log-file>");
            eprintln!("  cat <log-file> | singbox-log-doctor");
            return;
        }

        input
    };

    let mut total_lines = 0;
    let mut error_count = 0;
    let mut warn_count = 0;
    let mut dns_bad_rdata = 0;
    let mut dns_buffer_too_small = 0;
    let mut tailscale_timeout = 0;
    let mut endpoint_not_connected = 0;
    let mut google_ip_through_ts_out = 0;

    for line in content.lines() {
        total_lines += 1;

        if line.contains("ERROR") {
            error_count += 1;
        }

        if line.contains("WARN") {
            warn_count += 1;
        }

        if line.contains("bad rdata") {
            dns_bad_rdata += 1;
        }

        if line.contains("buffer size too small") {
            dns_buffer_too_small += 1;
        }

        if line.contains("i/o timeout") || line.contains("connection timed out") {
            tailscale_timeout += 1;
        }

        if line.contains("endpoint not connected") {
            endpoint_not_connected += 1;
        }

        if line.contains("142.250.") && line.contains("ts-out") {
            google_ip_through_ts_out += 1;
        }
    }

    println!("singbox-log-doctor report");
    println!("=========================");
    println!("Total lines: {}", total_lines);
    println!("Errors: {}", error_count);
    println!("Warnings: {}", warn_count);
    println!();

    println!("DNS issues:");
    println!("  bad rdata: {}", dns_bad_rdata);
    println!("  buffer size too small: {}", dns_buffer_too_small);
    println!();

    println!("Tailscale / ts-out issues:");
    println!("  timeout: {}", tailscale_timeout);
    println!("  endpoint not connected: {}", endpoint_not_connected);
    println!();

    println!("Routing issues:");
    println!("  Google IP through ts-out: {}", google_ip_through_ts_out);
    println!();

    println!("Advice:");
    if dns_bad_rdata > 0 || dns_buffer_too_small > 0 {
        println!("  - DNS packet errors detected. Check whether auto_redirect or DNS hijack is capturing abnormal LAN discovery traffic.");
    }

    if tailscale_timeout > 0 || endpoint_not_connected > 0 {
        println!("  - Tailscale endpoint errors detected. Check ts-out and the exit node status.");
    }

    if google_ip_through_ts_out > 0 {
        println!("  - Google IP appears to be routed through ts-out. Check geosite-cn / geoip-cn rules and add direct exceptions if needed.");
    }

    if dns_bad_rdata == 0
        && dns_buffer_too_small == 0
        && tailscale_timeout == 0
        && endpoint_not_connected == 0
        && google_ip_through_ts_out == 0
    {
        println!("  - No common issues detected.");
    }
}