use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use reqwest::header::{Connection, UserAgent};
use reqwest::Client;
use error::GrnEnvError;

pub fn file_download<'a>(
    client: &'a Client,
    url: &str,
    mut base_dir: PathBuf,
    filename: &'a str,
) -> Result<PathBuf, GrnEnvError> {
    let mut res = client
        .get(url)
        .header(Connection::close())
        .header(UserAgent::new(format!("grnenv-rs {}", crate_version!())))
        .send()?;
    println!("{:?}", res);
    let mut body = vec![];
    res.read_to_end(&mut body)?;
    base_dir.push(filename);
    let mut f = File::create(base_dir.to_str().unwrap()).expect("Could not create file.");
    println!("{:?}", f);
    f.write_all(&body)?;
    f.sync_data()?;
    Ok(base_dir)
}

pub fn page_download<'a>(client: &'a Client, url: &str) -> Result<String, GrnEnvError> {
    let mut res = client
        .get(url)
        .header(Connection::close())
        .header(UserAgent::new(format!("grnenv-rs {}", crate_version!())))
        .send()?;
    println!("{:?}", res);
    let mut body = vec![];
    res.read_to_end(&mut body)?;
    let contents = String::from_utf8_lossy(&body).to_string();
    Ok(contents)
}
