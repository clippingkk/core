use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CKError {
    details: String
}

impl CKError {
    pub fn new(msg: &str) -> CKError {
        CKError{details: msg.to_string()}
    }
}

impl fmt::Display for CKError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CKError {
    fn description(&self) -> &str {
        &self.details
    }
}