#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;

use alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::{
    entry_point,
    prelude::{Api, SystemApi},
    println,
};

fn main() {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "/test.html".to_string()) {
        Ok(res) => {
            println!("response:");
            println!("{:#?}", res);
        }
        Err(e) => {
            println!("error:");
            println!("{:#?}", e);
        }
    };
    Api::exit(0);
}

entry_point!(main);
