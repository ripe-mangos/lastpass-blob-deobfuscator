use lpass_blob::Results;
use std::{fs::File, io::Read};

fn main() {
    let fpath = std::env::args().skip(1).next().unwrap();

    let mut f = File::open(fpath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    
    let res = Results::try_from(buf.as_str()).unwrap();
    println!("Account version: {}\nAccount CBC: {}", res.version, if res.acct_cbc { "enabled" } else { "disabled" });
    
    let ecb_accts = res.accounts.iter().filter(|x| !x.login.cbc);
    if ecb_accts.clone().count() > 0 {
        println!("AES-ECB accounts found!");
        for a in ecb_accts {
            println!(" - {}", a.url);
        }
    } else {
        println!("No AES-ECB accounts found. :)");
    }
}
