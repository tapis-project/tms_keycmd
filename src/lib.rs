#![forbid(unsafe_code)]

use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::str;
use std::time::Duration;
//use anyhow::{Context, Result, anyhow};
// Start with ureq for simple http client calls.
// Using reqwest may also be a good choice. use reqwest::blocking;
//use ureq::{Agent, AgentBuilder, Response};
// TODO/TBD: use attohttpc
//use attohtpc;
use serde_json::{json, Value};
use hostname;
use local_ip_address::local_ip;
use serde::{Serialize, Deserialize};
use figment::{Figment, providers::{Format, Toml}};

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
    SshRsa
}
impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyType::SshRsa => write!(f, "ssh-rsa")
        }
    }
}
impl str::FromStr for KeyType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssh-rsa" => Ok(KeyType::SshRsa),
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
    pub keytype: String
//    pub keytype: KeyType // No need for this to be an enum. For info only
}

// ------------------------------------------
// ReqPubKey
// ------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct ReqPubKey {
    pub user: String,
    pub user_uid: String,
    pub user_home_dir: String,
    pub host: String,
    pub public_key_fingerprint: String,
    pub requestor_host: String,
    pub requestor_addr: String
//    pub keytype: KeyType // No need for this to be an enum. For info only
}

// ------------------------------------------
// Config
// ------------------------------------------
#[derive(Deserialize, Debug)]
pub struct Config {
    pub tms_url: String,
    pub host_name: String,
    pub client_id: String,
    pub client_secret: String
}

// ==========================================
// Functions
// ==========================================

// ------------------------------------------
// run
// Call TMS server and output result to stdout
// ------------------------------------------
pub fn run(cmd_args: CmdArgs) -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir()?;
    log::info!("Running in current working directory: {}", cwd.display());
    log::info!("Running with fingerprint: {}", cmd_args.fingerprint);

    // Read properties from a config file: tms_url, host_name, client_id, client_secret
    // All values are required
    let config: Config = Figment::new().merge(Toml::file("tms_keycmd.toml")).extract()?;
    log::info!("Using configuration - tms_url: {} host: {} client_id: {} client_secret: {}",
             config.tms_url, config.host_name, config.client_id, config.client_secret);
    // Check that we have all required config settings.
    if config.tms_url.trim().is_empty() { panic!("Configuration attribute must be set: tms_url") };
    if config.host_name.trim().is_empty() { panic!("Configuration attribute must be set: host_name") };
    if config.client_secret.trim().is_empty() { panic!("Configuration attribute must be set: client_id") };
    if config.client_id.trim().is_empty() { panic!("Configuration attribute must be set: client_secret") };

    // Get the local host name and IP address
    let local_host_name = hostname::get()?;
    let local_host_name_cow = local_host_name.to_string_lossy();
    let local_host_name_str = local_host_name_cow.to_string();
    log::info!("Found local hostname: {:?}", local_host_name_str);
    let local_host_ip = local_ip().unwrap();
    log::info!("Found local ip address: {}", local_host_ip);

    // Build the request body to be sent to the TMS server
    let req_pub_key = ReqPubKey {
        user: cmd_args.username,
        user_uid: cmd_args.userid.to_string(),
        user_home_dir: cmd_args.home_dir,
        host: config.host_name,
        public_key_fingerprint: cmd_args.fingerprint,
        requestor_host: local_host_name_str,
        requestor_addr: local_host_ip.to_string()
};
// //    let r = get_req_pub_key_from_args(cmd_args);
//     let req_pub_key = ReqPubKey {
//         user: "testhostaccount1".to_owned(),
//         user_uid: "1".to_owned(),
//         user_home_dir: "/home/testaccount1".to_owned(),
//         host: config.host_name.to_owned(),
//         public_key_fingerprint: "SHA256:+oGXmhj1nu4snzHHJQimX7q3s0o8M7NRaFbxV7+pvfE".to_owned(),
//         requestor_host: local_host_name_str,
//         requestor_addr: local_host_ip.to_string()
//     };

    let req_pub_key_str = serde_json::to_string(&req_pub_key)?;
/*
    {
        "user": "testhostaccount1",
        "user_uid": "1",
        "user_home_dir": "/home/testaccount1",
        "host": "testhost1",
        "public_key_fingerprint": "SHA256:+oGXmhj1nu4snzHHJQimX7q3s0o8M7NRaFbxV7+pvfE",
        "requestor_host": "localhost",
        "requestor_addr": "127.0.0.1"
    }
*/
    // // Create the http agent
    // let agent: Agent = AgentBuilder:: new()
    //     .timeout_read(Duration::from_secs(5))
    //     .timeout_write(Duration::from_secs(5))
    //     .build();

    // Send the post request
    log::info!("Sending json request body: {}", req_pub_key_str);
    let resp = attohttpc::post(&config.tms_url).json(&req_pub_key)?.send()?;
    if resp.is_success() {
        let resp_json: Value = resp.json()?;
        log::info!("Got resp_json: {}", resp_json);
        let pub_key_str = resp_json["public_key"].as_str().unwrap().trim();
        log::info!("Writing pubkey to stdout: {}", pub_key_str);
        print!("{}", pub_key_str);
        io::stdout().flush().unwrap();
//        println!("Body:\n{}", resp.text_utf8()?);
//        let body: Value = resp_json
    }
//     let resp = agent.post(&config.tms_url)
//          .send_json(ureq::json!(req_pub_key))?;
//     // Make the http request
//     // Simple form using anyhow and ? for error handling
//     // let resp: Response = agent.get("https://dev.develop.tapis.io/v3/systems/asdfdsfdsfasdf")
//     //     .set("X-Tapis-Token", "jwt_asldfkjdfj")
//     //     .call().with_context(|| format!("Blah {}", "blah"))?;
//     // Simple form using ? for error handling
//     // let resp: Response = agent.get("https://dev.develop.tapis.io/v3/systems/healthcheck")
//     //     .set("X-Tapis-Token", "jwt_asldfkjdfj")
//     //     .call()?;
//     // Verbose form with explicit error handling
// //     let resp: Response = match agent.get("https://dev.develop.tapis.io/v3/systems/healthcheck")
// //         .set("X-Tapis-Token", "jwt_asldfkjdfj")
// //         .call() {
// //             Ok(response) => {response},
// //             Err(error) => { return Err(error.into()); }
// //             // Err(ureq::Error::Status(code, response)) => {
// //             //     // Server returned a non-200 error code, such as 400, 500, etc
// //             // },
// // //            Err(_) => { /* some other error */ return Err("failed") }
// //         };



    // let body = resp.into_string()?;
    // let body_json = resp.into_json()?;
    // log::info!("Got response: {}", resp.into_json().unwrap().);
    // println!("{}", bod.)
    // // TODO Write public key to stdout
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
    log::info!("Program = {}", arg0);
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
//    let keytype_str = args[5].clone();

    // Log arguments
    log::info!("username={username} userid={userid_str} home_dir={home_dir} keytype={keytype}");
    log::info!("fingerprint={fingerprint}");

    // Parse 2nd argument as userid. It must be a number
    let userid: u32 = match userid_str.trim().parse() {
        Ok(num) => num,
        Err(_) => { return Err("userid must be a number") }
    };
    // // Parse 5th argument as a KeyType
    // let keytype: KeyType = keytype_str.trim().parse().unwrap();

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
//        assert!(cmd_args.keytype == KeyType::SshRsa);
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
