#![forbid(unsafe_code)]

use std::error::Error;

// ****************************************************************************
// Library code for program keycmd
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
// Constants
// -----------------------------------
pub const USAGE : &str = "Usage: keycmd <username> <userid> <home_dir> <fingerprint> <keytype>";

// -----------------------------------
// Structures
// -----------------------------------
pub struct CmdArgs {
    pub username: String,
    pub userid: u32,
    pub home_dir: String,
    pub fingerprint: String,
    pub keytype: String
}

// -----------------------------------
// Functions
// -----------------------------------

//
// run
// Call TMS server and output result to stdout
// 
pub fn run(cmd_args: CmdArgs) -> Result<(), Box<dyn Error>> {
    println!("Running with fingerprint: {}", cmd_args.fingerprint);
    Ok(())
}

//
// parse_args
// Process the command line arguments
// 
pub fn parse_args(args: &[String]) -> Result<CmdArgs, &'static str>  {
    let arg0 = args[0].clone();
    println!("Program = {}", arg0);
    // Check number of arguments
    if args.len() != 6 {
        return Err("Incorrect number of arguments. Please provide 5 arguments.");
    }
    // NOTE Use clone for clarity. Could be done faster and more efficiently without clone,
    //  but here such concerns are not critical and clone is more straightforward.
    let username = args[1].clone();
    let userid_str = args[2].clone();
    let home_dir = args[3].clone();
    let fingerprint = args[4].clone();
    let keytype = args[5].clone();

    // Log arguments
    println!("username={username} userid={userid_str} home_dir={home_dir} keytype={keytype}");
    println!("fingerprint={fingerprint}");

    // Parse 2nd argument as userid. It must be a number
//    let userid1: u32 = userid_str.trim().parse()?; Why is this incorrect?
    let userid: u32 = match userid_str.trim().parse() {
        Ok(num) => num,
        Err(_) => { return Err("userid must be a number") }
    };

    Ok(CmdArgs { username, userid, home_dir, fingerprint, keytype })
}