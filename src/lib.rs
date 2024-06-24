#![forbid(unsafe_code)]

use std::env;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::str;
use anyhow::{Result, anyhow};
use figment::{Figment, providers::{Format, Toml}};
use fs_mistrust::Mistrust;
use hostname;
use local_ip_address::local_ip;
use path_absolutize::Absolutize;
use serde::{Serialize, Deserialize};
use serde_json::Value;

// ****************************************************************************
// Library code for program keycmd
// Command line program to support the SSH AuthorizedKeysCommand option for
// retrieving authorized public keys for a user during ssh login.
// 
// This program accepts 4 arguments and calls the Trust Manager System (TMS)
// server to fetch the associated public key. The public key is written
// to stdout.
// If no public key is found then nothing is written to stdout.
//
// The following 4 arguments must be passed in on the command line:
//     %u - login username
//     %U - numeric login user id
//     %f - fingerprint of the public key to be fetched
//     %t - ssh key type
// Example:
//   keycmd jdoe 1001 SHA256:I/YLbfco8m4WWZSDSNZ/OnV26tt+BgtFAcAb94Co974 ssh-rsa
// 
// ****************************************************************************

// ==========================================
// Constants
// ==========================================

pub const USAGE : &str = "Usage: keycmd <username> <userid> <fingerprint> <keytype>";
pub const CFG_FILE : &str = "tms_keycmd.toml";
pub const LOG_CFG_FILE : &str = "log4rs.yml";

// ==========================================
// Enumerations
// ==========================================

// ==========================================
// Structures
// ==========================================

// ------------------------------------------
// CmdArgs
// ------------------------------------------
pub struct CmdArgs {
    pub username: String,
    pub userid: u32,
    pub fingerprint: String,
    pub keytype: String
}

// ------------------------------------------
// ReqPubKey
// ------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct ReqPubKey {
    pub user: String,
    pub user_uid: String,
    pub host: String,
    pub public_key_fingerprint: String,
    pub key_type: String
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
// Public functions
// ==========================================

// -----------------------------------
//   tms_init
//   Initialize tms_keycmd. If init fails log an error and return false.
// -----------------------------------
pub fn tms_init() -> bool {
    // TODO Check that config files exist and have correct permissions   

    // Initialize logger
    // On error write to stderr using eprintln
    match log4rs::init_file(LOG_CFG_FILE, Default::default()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return false;
        }
    }
    true
}
// ------------------------------------------
// run
// Call TMS server and output result to stdout
// On success return true
// On error log message and return false
// ------------------------------------------
pub fn run(cmd_args: CmdArgs) -> bool {
    let cwd = match env::current_dir() {
        Ok(path_buf) => path_buf,
        Err(error) => {
            log::error!("Unable to determine current directory. Error: {error:?}");
            return false
        }
    };
    log::info!("Running in current working directory: {}", cwd.display());
    log::info!("Running with fingerprint: {}", cmd_args.fingerprint);

    // Read properties from a config file: tms_url, host_name, client_id, client_secret
    // All values are required
    let config: Config =  match Figment::new().merge(Toml::file(CFG_FILE)).extract() {
        Ok(c) => c,
        Err(error) => {
            log::error!("Error reading config file. File: {CFG_FILE:?}. Error: {error:?}");
            return false
        } 
    };
    log::info!("Using configuration - tms_url: {} host: {} client_id: {} client_secret: {}",
               config.tms_url, config.host_name, config.client_id, config.client_secret);
    // Check that we have all required config settings.
    if config.tms_url.trim().is_empty() { log::error!("Configuration attribute must be set: tms_url"); return false };
    if config.host_name.trim().is_empty() { log::error!("Configuration attribute must be set: host_name"); return false };
    if config.client_secret.trim().is_empty() { log::error!("Configuration attribute must be set: client_id"); return false };
    if config.client_id.trim().is_empty() { log::error!("Configuration attribute must be set: client_secret"); return false };

    // Get the local host name and IP address
    let local_host_name = match hostname::get() {
        Ok(h) => h,
        Err(error) => {
            log::error!("Error determining local host name. Error: {error:?}");
            return false
        }
    };
    let local_host_name_cow = local_host_name.to_string_lossy();
    let local_host_name_str = local_host_name_cow.to_string();
    log::info!("Found local hostname: {:?}", local_host_name_str);
    let local_host_ip = match local_ip() {
        Ok(ipaddr) => ipaddr,
        Err(error) => {
            log::error!("Error determing local IP address: {error}");
            return false    
        }
    };
    log::info!("Found local ip address: {}", local_host_ip);

    // Build the request body to be sent to the TMS server
    let req_pub_key = ReqPubKey {
        user: cmd_args.username,
        user_uid: cmd_args.userid.to_string(),
        host: config.host_name,
        public_key_fingerprint: cmd_args.fingerprint,
        key_type: cmd_args.keytype
    };

    // Send the post request and extract the public key from the response
    let pub_key_str = match send_request(&req_pub_key, &config.tms_url) {
        Ok(pub_key) => pub_key,
        Err(error) => {
            log::error!("Error retrieving public key from TMS server. Error: {error:?}");
            return false
        }
    };
    // Write the public key to stdout
    log::info!("Writing public key to stdout: {}", pub_key_str);
    print!("{}", pub_key_str);
    match io::stdout().flush() {
        Ok(()) => return true,
        Err(error) => {
            log::error!("Error flushing stdout. Error: {error}");
            return false
        }
    }
}

// ------------------------------------------
// parse_args
// Process the command line arguments
// ------------------------------------------
pub fn parse_args(args: &[String]) -> Result<CmdArgs, &'static str>  {
    let arg0 = args[0].clone();
    log::info!("Program = {}", arg0);
    // Check number of arguments
    if args.len() != 5 {
        return Err("Incorrect number of arguments. Please provide 4 arguments.");
    }
    // NOTE Use clone for clarity. Could be done faster and more efficiently without clone,
    //  but here such concerns are not critical and clone is more straightforward.
    let username = args[1].clone();
    let userid_str = args[2].clone();
    let fingerprint = args[3].clone();
    let keytype = args[4].clone();

    // Log arguments
    log::info!("username={username} userid={userid_str} keytype={keytype}");
    log::info!("fingerprint={fingerprint}");

    // Parse 2nd argument as userid. It must be a number
    let userid: u32 = match userid_str.trim().parse() {
        Ok(num) => num,
        Err(_) => { return Err("userid must be a number") }
    };

    Ok(CmdArgs { username, userid, fingerprint, keytype })
}

// ==========================================
// Private functions
// ==========================================

// ------------------------------------------
// send_request
// Send the post request and extract the public key from the response
// ------------------------------------------
pub fn send_request(req_pub_key: &ReqPubKey, tms_url: &String) -> Result<String> {
    let req_pub_key_str = serde_json::to_string(&req_pub_key)?;
    log::info!("Sending json request body: {}", req_pub_key_str);
    let resp = attohttpc::post(tms_url).json(&req_pub_key)?.send()?;
    // Find out result status. Do this before getting json because getting json moves the Response value.
    let resp_ok = resp.is_success();
    // Convert the response to json and log it
    let resp_json: Value = resp.json()?;
    log::info!("Received json response: {}", resp_json);
    if !resp_ok {
         return Result::Err(anyhow!("Request not successful. Please see response received."));
    } else {
        // Extract the public key from the json and return it as a String
        let pub_key_str = match resp_json["public_key"].as_str() {
            Some(s) => s,
            None => return Result::Err(anyhow!("Unable to find public key in json response. Please see response received."))
        };
        return Result::Ok(pub_key_str.to_string());
    }
}

// ------------------------------------------
// get_mistrust
// Construct a mistrust object for file permission checks
// ------------------------------------------
fn get_mistrust() -> Mistrust {
    let mistrust = match Mistrust::builder() 
        .ignore_prefix(get_absolute_path("./"))
        .trust_group(0)
        .build() {
            Ok(m) => m,
            Err(e) => {
                panic!("Mistrust configuration error: {}", &e.to_string());
            }
        };
    mistrust
}

// ------------------------------------------
// get_absolute_path
// Replace ~ and environment variables using shellexpand.
// On error, return the original input string.
// ------------------------------------------
pub fn get_absolute_path(path: &str) -> String {
    // Use shellexpand to resolve ~ and env variables
    let s = match shellexpand::full(path) {
        Ok(x) => x,
        Err(_) => return path.to_owned(),
    };
    // Use absolutize to convert to absolute path.
    let p = Path::new(s.deref());
    let p1 = match p.absolutize() {
        Ok(x) => x,
        Err(_) => return path.to_owned(),
    };
    let p2 = match p1.to_str() {
        Some(x) => x,
        None => return path.to_owned(),
    };
    p2.to_owned()
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
                                     "abc_fingerprint_def".to_string(),
                                     "ssh-key".to_string()];
        let cmd_args = parse_args(okay_args).unwrap();
        assert_eq!(cmd_args.username, "jdoe");
        assert_eq!(cmd_args.userid, 1111);
        assert_eq!(cmd_args.fingerprint, "abc_fingerprint_def");
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
                                     "abc_fingerprint_def".to_string(),
                                     "ssh-key".to_string()];
        let _cmd_args = match parse_args(bad_userid_args) {
            Ok(_) => panic!("ERROR: Call with invalid userid string should fail."),
            Err(error) => assert!(error.contains("userid must be a number"))
        };
    }
}
