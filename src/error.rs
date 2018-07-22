use reqwest::Error as RequestError;
use std::io;

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
