use std::error::Error;
use std::time::Instant;
pub mod gpus;
pub mod systeminfo;
pub mod displayinfo;


pub fn run() -> Result<(), Box<dyn Error>>{
    let start = Instant::now();
    let (display_info, monitor) = displayinfo::displayinfo();
    let gpu = gpus::get_gpus();
    let sysinfo = systeminfo::systeminfo();
    
    println!("System Name: {}", sysinfo.sys_name());
    println!("Host Name: {}", sysinfo.host_name());
    println!("Uptime: {:?}", start.elapsed());
    println!("{}", display_info);
    println!("{}", monitor);
    println!("CPU: {}", sysinfo.cpu_name());
    println!("GPU: {}", gpu);
    println!("Memory: {:.2} GiB / {:.2} GiB", sysinfo.used_ram(), sysinfo.total_ram());
    Ok(())
}