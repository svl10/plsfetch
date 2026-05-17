use std::error::Error;
use std::time::Instant;
pub mod gpus;
pub mod systeminfo;
pub mod displayinfo;
pub mod ascii;

pub fn run() -> Result<(), Box<dyn Error>>{
    let start = Instant::now();
    let (display_info, monitor) = displayinfo::displayinfo();
    let gpu = gpus::get_gpus();
    let sysinfo = systeminfo::systeminfo();
    
    let system_name = String::from(format!("System Name: {}", sysinfo.sys_name()));
    let host_name = String::from(format!("Host Name: {}", sysinfo.host_name())); 
    let fetch_time = String::from(format!("Fetch Time: {:?}", start.elapsed()));
    let display_info = String::from(format!("{}", display_info));
    let monitor = String::from(format!("{}", monitor));
    let cpu = String::from(format!("CPU: {}", sysinfo.cpu_name()));
    let gpu = String::from(format!("GPU: {}", gpu));
    let memory = String::from(format!("Memory: {:.2} GiB / {:.2} GiB", sysinfo.used_ram(), sysinfo.total_ram()));
    
    let info = vec![
        system_name,
        host_name,
        fetch_time,
        display_info,
        monitor,
        cpu,
        gpu,
        memory
    ];
    let (ascii_art, mut width) = ascii::cat_sit();
    width += 3;

    for i in 0..ascii_art.len().max(info.len()){
        let left_side = ascii_art.get(i).map_or("", |v| v);
        let right_side = info.get(i).map_or("", |v| v);
        
        println!("{:<width$} | {}", left_side, right_side, width=width);
    }



    Ok(())
}