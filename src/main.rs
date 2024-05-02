#![forbid(unsafe_code)]

use std::env;

// ****************************************************************************
// Program keycmd
// Command line program to support the SSH AuthorizedKeysCommand option for
// retrieving authorized public keys for a user during ssh login.
// 
// This program accepts 5 arguments and calls the Trust Manager System (TMS)
// server to fetch the associated public key. The public key is written
// to stdout.
// If no public key is found then TBD
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
// Constants
// -----------------------------------
const USAGE : &str = "Usage: keycmd <username> <userid> <home_dir> <fingerprint> <keytype>";

// -----------------------------------
// Main
// -----------------------------------
fn main() {
    println!("TMS keycmd v0.0.1");
    let args: Vec<String> = env::args().collect();
    let cmd_args = parse_args(&args).expect(USAGE);
}

// -----------------------------------
// Structures
// -----------------------------------
struct CmdArgs {
    username: String,
    userid: u32,
    home_dir: String,
    fingerprint: String,
    keytype: String
}

// -----------------------------------
// Functions
// -----------------------------------

//
// parse_args
// Process the command line arguments
// 
fn parse_args(args: &[String]) -> Result<CmdArgs, &'static str>  {
    let arg0 = args[0].clone();
    println!("Program = {}", arg0);
    // Check number of arguments
    if args.len() != 6 {
        return Err("Incorrect number of arguments. Please provide 5 arguments.");
    }
    // NOTE Use clone for clarity. Could be done faster more efficiently without clone,
    //  but here such concerns are not critical and clone is more straightforward.
    let username = args[1].clone();
    let userid_str = args[2].clone();
    let home_dir = args[3].clone();
    let fingerprint = args[4].clone();
    let keytype = args[5].clone();

    // Log arguments
    println!("username={username} userid={userid_str} home_dir={home_dir} keytype={keytype}");
    println!("fingerprint={fingerprint}");

    // Parse 2nd argument as userid, so it must be a number
    let userid: u32 = match userid_str.trim().parse() {
        Ok(num) => num,
        Err(_) => { return Err("userid must be a number") }
    };

    Ok(CmdArgs { username, userid, home_dir, fingerprint, keytype })
}