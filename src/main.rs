#![forbid(unsafe_code)]

use std::env;
use std::process;
use tms_keycmd::{self}; // Include everything from lib.rs

// ****************************************************************************
// Program keycmd
// Command line program to support the SSH AuthorizedKeysCommand option for
// retrieving authorized public keys for a user during ssh login.
// 
// This program accepts 5 arguments and calls the Trust Manager System (TMS)
// server to fetch the associated public key. The public key is written
// to stdout.
// If no public key is found then nothing is written to stdout.
//
// The following 5 arguments must be passed in on the command line:
//     %u - login username
//     %U - numeric login user id
//     %h - home directory of login user
//     %f - fingerprint of the public key to be fetched
//     %t - ssh key type
// Example:
//   keycmd jdoe 1001 /home/jdoe SHA256:I/YLbfco8m4WWZSDSNZ/OnV26tt+BgtFAcAb94Co974 ssh-rsa
// 
// ****************************************************************************

// -----------------------------------
// Main
// -----------------------------------
fn main() {
    println!("TMS keycmd v0.0.1");
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let cmd_args = tms_keycmd::parse_args(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        println!("Usage: {}", tms_keycmd::USAGE);
        process::exit(1);
    });

    println!("Calling TMS server using: username={}, userid={}, home_dir={}, fingerprint={}, keytype={}",
             cmd_args.username, cmd_args.userid, cmd_args.home_dir, cmd_args.fingerprint, cmd_args.keytype);
    // Run the main code and print error message if it fails
    if let Err(e) = tms_keycmd::run(cmd_args) {
        println!("Program error: {e}");
        process::exit(1);
    }
}
