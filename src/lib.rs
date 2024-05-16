#![forbid(unsafe_code)]

use std::error::Error;
//use anyhow::{Context, Result, anyhow};
// Start with ureq for simple http client calls.
// Using reqwest may also be a good choice.
use ureq::{Agent, AgentBuilder, Response};
use std::time::Duration;
use std::fmt;
use std::str;

// use reqwest::blocking;

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

// ==========================================
// Constants
// ==========================================
pub const USAGE : &str = "Usage: keycmd <username> <userid> <home_dir> <fingerprint> <keytype>";

// ==========================================
// Enumerations
// ==========================================
// ------------------------------------------
// KeyType
// ------------------------------------------
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum KeyType {
    SshKey
}
impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyType::SshKey => write!(f, "ssh-key")
        }
    }
}
impl str::FromStr for KeyType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssh-key" => Ok(KeyType::SshKey),
            _ => Err(format!("'{}' is not a valid key type", s))
        }
    }
}

// ==========================================
// Structures
// ==========================================
// ------------------------------------------
// CmdArgs
// ------------------------------------------
pub struct CmdArgs {
    pub username: String,
    pub userid: u32,
    pub home_dir: String,
    pub fingerprint: String,
    pub keytype: KeyType
}

// ==========================================
// Functions
// ==========================================

// ------------------------------------------
// run
// Call TMS server and output result to stdout
// ------------------------------------------
pub fn run(cmd_args: CmdArgs) -> Result<(), Box<dyn Error>> {
    println!("Running with fingerprint: {}", cmd_args.fingerprint);

    // Create the http agent
    let agent: Agent = AgentBuilder:: new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    // Make the http request
    // Simple form using anyhow and ? for error handling
    // let resp: Response = agent.get("https://dev.develop.tapis.io/v3/systems/asdfdsfdsfasdf")
    //     .set("X-Tapis-Token", "jwt_asldfkjdfj")
    //     .call().with_context(|| format!("Blah {}", "blah"))?;
    // Simple form using ? for error handling
    let resp: Response = agent.get("https://dev.develop.tapis.io/v3/systems/healthcheck")
        .set("X-Tapis-Token", "jwt_asldfkjdfj")
        .call()?;
    // Verbose form with explicit error handling
//     let resp: Response = match agent.get("https://dev.develop.tapis.io/v3/systems/healthcheck")
//         .set("X-Tapis-Token", "jwt_asldfkjdfj")
//         .call() {
//             Ok(response) => {response},
//             Err(error) => { return Err(error.into()); }
//             // Err(ureq::Error::Status(code, response)) => {
//             //     // Server returned a non-200 error code, such as 400, 500, etc
//             // },
// //            Err(_) => { /* some other error */ return Err("failed") }
//         };
    let body = resp.into_string()?;
    println!("Got systems healthcheck: {}", body);
    Ok(())
}

//
// ------------------------------------------
// parse_args
// Process the command line arguments
// ------------------------------------------
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
    let keytype_str = args[5].clone();

    // Log arguments
    println!("username={username} userid={userid_str} home_dir={home_dir} keytype={keytype_str}");
    println!("fingerprint={fingerprint}");

    // Parse 2nd argument as userid. It must be a number
    let userid: u32 = match userid_str.trim().parse() {
        Ok(num) => num,
        Err(_) => { return Err("userid must be a number") }
    };
    // Parse 5th argument as a KeyType
    let keytype: KeyType = keytype_str.trim().parse().unwrap();
    // let keytype: KeyType = match keytype_str.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => { return Err("userid must be a number") }
    // };

    Ok(CmdArgs { username, userid, home_dir, fingerprint, keytype })
}

// ==========================================
// Unit tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test with valid arguments
    #[test]
    fn test_okay() {
        let okay_args: &[String] = &["keycmd".to_string(), "jdoe".to_string(), "1111".to_string(),
                                     "/home/jdoe".to_string(), "abc_fingerprint_def".to_string(),
                                     "ssh-key".to_string()];
        let cmd_args = parse_args(okay_args).unwrap();
        assert_eq!(cmd_args.username, "jdoe");
        assert_eq!(cmd_args.userid, 1111);
        assert_eq!(cmd_args.home_dir, "/home/jdoe");
        assert_eq!(cmd_args.fingerprint, "abc_fingerprint_def");
        assert!(cmd_args.keytype == KeyType::SshKey);
    }

    // Test with too many arguments
    #[test]
    fn test_too_many_args() {
        let many_args: &[String] = &["keycmd".to_string(), "a1".to_string(), "a2".to_string(), "a3".to_string(),
                                     "a4".to_string(), "a5".to_string(), "a6".to_string()];
        let _cmd_args = match parse_args(many_args) {
            Ok(_) => panic!("ERROR: Call with too many arguments should fail."),
            Err(error) => assert!(error.contains("Incorrect number of arguments"))
        };
    }

    // Test with too few arguments
    #[test]
    fn test_too_few_args() {
        let few_args: &[String] = &["keycmd".to_string(), "a1".to_string(), "a2".to_string()];
        let _cmd_args = match parse_args(few_args) {
            Ok(_) => panic!("ERROR: Call with too few arguments should fail."),
            Err(error) => assert!(error.contains("Incorrect number of arguments"))
        };
    }

    // Test with invalid userid argument
    #[test]
    fn test_userid_not_num() {
        let bad_userid_args: &[String] = &["keycmd".to_string(), "jdoe".to_string(), "1111a".to_string(),
                                     "/home/jdoe".to_string(), "abc_fingerprint_def".to_string(),
                                     "ssh-key".to_string()];
        let _cmd_args = match parse_args(bad_userid_args) {
            Ok(_) => panic!("ERROR: Call with invalid userid string should fail."),
            Err(error) => assert!(error.contains("userid must be a number"))
        };
    }
}