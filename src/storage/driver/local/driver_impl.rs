
use std::{fmt::Display, fs::{self, File}, path};

use thiserror::Error;

use crate::storage::driver::{Driver, DriverError};

pub type Result<T> = std::result::Result<T,LocalDriverError>;

#[derive(Debug,Error)]
pub struct LocalDriverError(#[from]std::io::Error);

impl Display for LocalDriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}


pub struct Local {}

impl Driver for Local {

    fn write_file(&self,path: &str) -> Result<()> {
        
       let a =File::open(path)?;

       Ok(())
    }

    fn read_file(&self,_path: &str) -> Result<Vec<u8>> {
      Ok(Vec::new())
    }
}