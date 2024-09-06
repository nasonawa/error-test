use std::error::Error;

use super::driver::Driver;



pub struct Storage {
    driver: Box<dyn Driver>
}

pub fn new(driver:  Box<dyn Driver>) -> Storage {Storage{driver}}

impl Storage {

pub fn write(&self,path:&str) -> Result<(), Box<dyn Error>> {
    Ok(self.driver.write_file(path)?)
}

pub fn read(&self,path:&str)-> Result<Vec<u8>, Box<dyn Error>> {
    Ok(self.driver.read_file(path)?)
} 

}