use std::error::Error;
use std::time::Instant;
pub mod systeminfo;
pub mod displayinfo;


pub fn run() -> Result<(), Box<dyn Error>>{
    let start = Instant::now();

    let display_info = displayinfo::displayinfo();

    Ok(())
}