//! # LastPass Blob Deobfuscator
//! 
//! A tool to deobfuscate your LastPass blob and determine how vulnerable you are with the recent
//! breach.
//!
//! Created because of Steve's call-to-action on the [Security Now! Podcast](https://grc.com/sn)

extern crate serde;
extern crate serde_xml_rs;

extern crate regex;
extern crate hex;
extern crate base64;

use std::error::Error;

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    accounts: Accounts,
}

#[derive(Deserialize)]
struct Accounts {
    accts_version: u8,
    cbc: bool,
    #[serde(rename = "$value")]
    accounts: Vec<Account>,
}

#[derive(Deserialize, Clone)]
pub struct Account {
    pub url: String,
    pub login: Login,
}

#[derive(Deserialize, Clone)]
pub struct Login {
    #[serde(skip)]
    pub cbc: bool,
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "p")]
    pub password: String,
}

/// LPBD's findings
#[derive(Clone)]
pub struct Results {
    pub version: u8,
    pub acct_cbc: bool,
    pub accounts: Vec<Account>,
}
impl TryFrom<&str> for Results {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data: Response = serde_xml_rs::from_str(value).unwrap();
    
        let version = data.accounts.accts_version;
        let acct_cbc = data.accounts.cbc;
        let accounts = data.accounts.accounts.iter().map(|x| {
            let url = core::str::from_utf8(hex::decode(x.url.clone()).unwrap().as_slice()).unwrap().to_string();
            let cbc = Regex::new("^![^|]*|.*$").unwrap().is_match(x.login.password.clone().as_str());
            let login = Login {
                cbc,
                ..x.login.clone()
            };

            Account { url, login }
        }).collect::<Vec<Account>>();

        Ok(Self {
            version,
            acct_cbc,
            accounts,
        })
    }
}

