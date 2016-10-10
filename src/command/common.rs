use std::fs;
use std::path::Path;

use kuchiki::traits::*;
use hyper::{Client, Url};
use hyper::client::Response;
use hyper::Result as HyperResult;
use util;

pub fn versions() {
    let groonga_versioned_dir = util::obtain_groonga_versioned_path();
    let paths = fs::read_dir(&Path::new(&groonga_versioned_dir)).unwrap();

    let names = paths.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    println!("Installed Groonga:");
    println!("\tsystem");
    for entry in names {
        println!("\t{}", entry);
    }
}

pub struct MaybeProxyUrl {
    pub url: Url,
}

impl<'a> IntoResponse for MaybeProxyUrl {
    fn into_response(self) -> HyperResult<Response> {
        extern crate env_proxy;
        let maybe_proxy = env_proxy::for_url(&self.url);

        let client = match maybe_proxy {
            None => Client::new(),
            Some(host_port) => Client::with_http_proxy(host_port.0, host_port.1),
        };
        client.get(self.url).send()
    }
}
