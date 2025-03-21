#![forbid(unsafe_code)]

use std::env;
use std::process;
use tms_keycmd::{self}; // Include everything from lib.rs

// ****************************************************************************
// Program keycmd
// Command line program to support the SSH AuthorizedKeysCommand option for
// retrieving authorized public keys for a user during ssh login.
// 
// This program accepts 4 arguments and calls the Trust Manager System (TMS)
// server to fetch the associated public key.
// If a public key is found it is written to stdout.
// If no public key is found then nothing is written to stdout.
// All other output is written to the log file.
//
// The following 4 arguments must be passed in on the command line:
//     %u - login username (used in key lookup)
//     %U - numeric login user id (info only)
//     %f - fingerprint of the public key to be fetched (used in key lookup)
//     %t - ssh key type (info only)
// Example:
//   keycmd jdoe 1001 SHA256:I/YLbfco8m4WWZSDSNZ/OnV26tt+BgtFAcAb94Co974 ssh-rsa
// 
// ****************************************************************************

// ==========================================
// Constants
// ==========================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// -----------------------------------
// Main
// -----------------------------------
fn main() {

    // Collect command line arguments. If first argument is --version then print version and exit
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1].clone().trim() == "--version" {
        println!("TMS keycmd v{}", VERSION);
        process::exit(0);
    }

    // Check config and initialize. If init fails it will log an error and return false.
    if !tms_keycmd::tms_init() { process::exit(1); }

    // Log startup and collect command line arguments
    log::info!("TMS keycmd v{}", VERSION);

    // Parse command line arguments
    let cmd_args = tms_keycmd::parse_args(&args).unwrap_or_else(|err| {
        log::error!("Error parsing arguments: {err}");
        log::error!("Usage: {}", tms_keycmd::USAGE);
        process::exit(1);
    });

    log::debug!("Calling TMS server using: username={}, userid={}, fingerprint={}, keytype={}",
                cmd_args.username, cmd_args.userid, cmd_args.fingerprint, cmd_args.keytype);
    // Run the main code. If it fails it will log error message and return false
    if !tms_keycmd::run(cmd_args) { process::exit(1); }
    process::exit(0);
}

// ==========================================
// Private functions
// ==========================================
