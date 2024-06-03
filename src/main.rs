#![forbid(unsafe_code)]

use std::env;
use std::process;
use log::SetLoggerError;
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

// -----------------------------------
// Main
// -----------------------------------
fn main() -> Result<(), SetLoggerError> {
    // Initialize logger
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    log::info!("TMS keycmd v0.0.1");
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let cmd_args = tms_keycmd::parse_args(&args).unwrap_or_else(|err| {
        log::error!("Error parsing arguments: {err}");
        log::error!("Usage: {}", tms_keycmd::USAGE);
        process::exit(1);
    });

    log::info!("Calling TMS server using: username={}, userid={}, fingerprint={}, keytype={}",
             cmd_args.username, cmd_args.userid, cmd_args.fingerprint, cmd_args.keytype);
    // Run the main code and log error message if it fails
    if let Err(e) = tms_keycmd::run(cmd_args) {
        log::error!("Program error: {e}");
        process::exit(1);
    }
    Ok(())
}
