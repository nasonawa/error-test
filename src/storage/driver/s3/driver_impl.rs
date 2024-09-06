use crate::storage::driver::{Driver, DriverError};



pub struct S3 {}

impl Driver for S3 {

    fn write_file(&self,_path: &str) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
       Ok(())
    }

    fn read_file(&self,_path: &str) -> std::result::Result<Vec<u8>, Box<(dyn std::error::Error + 'static)>> {
      Ok(Vec::new())
    }
}