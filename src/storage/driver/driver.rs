use std::{error::Error, fmt::Display};

use thiserror::Error;

use super::LocalDriverError;

pub type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;

#[derive(Debug,Error)]
pub struct DriverError {
    kind: Errorkind,
    source: Box<dyn std::error::Error>
}

#[derive(Debug)]
pub enum Errorkind {
    Local,
    S3
}

impl Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<LocalDriverError> for DriverError {
    fn from(value: LocalDriverError) -> Self {
        DriverError{
            source:
            Box::new(value),
            kind: Errorkind::Local,
        }
    }
}


pub trait Driver {
    
    fn write_file(&self,path: &str) -> Result<()>;

    fn read_file(&self,path: &str) -> Result<Vec<u8>>;
 }