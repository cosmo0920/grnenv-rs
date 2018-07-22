use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use reqwest::Error as RequestError;
use reqwest::Client;
use reqwest::header::{Connection, UserAgent};

#[derive(Debug)]
pub enum GrnEnvError {
    ReqwestError(RequestError),
    IO(io::Error),
    #[doc(hidden)]
    Dummy(String),
}

impl From<RequestError> for GrnEnvError {
    fn from(err: RequestError) -> GrnEnvError {
        GrnEnvError::ReqwestError(err)
    }
}

impl From<io::Error> for GrnEnvError {
    fn from(err: io::Error) -> GrnEnvError {
        GrnEnvError::IO(err)
    }
}

pub fn file_download<'a>(client: &'a Client,
                         url: &str,
                         mut base_dir: PathBuf,
                         filename: &'a str)
                         -> Result<PathBuf, GrnEnvError> {

    let mut res = client.get(url)
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

pub fn page_download<'a>(client: &'a Client,
                         url: &str)
                         -> Result<String, GrnEnvError> {

    let mut res = client.get(url)
        .header(Connection::close())
        .header(UserAgent::new(format!("grnenv-rs {}", crate_version!())))
        .send()?;
    println!("{:?}", res);
    let mut body = vec![];
    res.read_to_end(&mut body)?;
    let contents = String::from_utf8_lossy(&body).to_string();
    Ok(contents)
}
