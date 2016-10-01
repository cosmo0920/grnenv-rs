use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use hyper::Client;
use hyper::error::Error as HyperError;
use hyper::header::{Connection, UserAgent};

pub fn file_download<'a>(client: &'a Client,
                         url: &str,
                         mut base_dir: PathBuf)
                         -> Result<PathBuf, HyperError> {

    let mut res = try!(client.get(url)
        .header(Connection::close())
        .header(UserAgent("grnenv-rs 0.1.0".to_string()))
        .send());
    println!("{:?}", res);
    let mut body = vec![];
    try!(res.read_to_end(&mut body));
    base_dir.push("groonga.zip");
    let mut f = File::create(base_dir.to_str().unwrap()).expect("Could not create file.");
    println!("{:?}", f);
    try!(f.write_all(&body));
    try!(f.sync_data());
    Ok(base_dir)
}
