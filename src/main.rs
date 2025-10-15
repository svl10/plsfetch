use sysinfo::{
    System, Pid,
};
use gfx_hal::{
    adapter::Adapter,
    Instance
};
use gfx_backend_vulkan as backend;
use display_info::DisplayInfo;
use std::time::Instant;

fn get_gpus() -> Vec<String> {

let instance: gfx_backend_vulkan::Instance =
backend::Instance::create("hayabusa", 1).unwrap();
let adapters: Vec<Adapter<backend::Backend>> = instance.enumerate_adapters();

let mut names: Vec<String> = Vec::new();

for adapter in adapters {
names.push(adapter.info.name.to_string());
}

names
}


struct Ram {
    base_total_ram: u64,
    base_used_ram: u64,
    total_ram: f64,
    used_ram: f64,
    metric_prefix: String,
}

impl Ram {

    fn total_ram(&self) -> f64 {
        self.total_ram
    }
    fn used_ram(&self) -> f64 {
        self.used_ram
    }

    fn set_to_mb(&mut self) {
        self.total_ram = self.base_total_ram as f64 / 1024.0 / 1024.0;
        self.used_ram = self.base_used_ram as f64 / 1024.0 / 1024.0;
        self.metric_prefix = String::from("MB");
    }

    fn set_to_gb(&mut self) {
        self.total_ram = self.base_total_ram as f64 / 1024.0 / 1024.0 / 1024.0;
        self.used_ram = self.base_used_ram as f64 / 1024.0 / 1024.0 / 1024.0;
        self.metric_prefix = String::from("GB");
    }


}

fn main() {
    let start = Instant::now();

    let mut sys = System::new_all();
    sys.refresh_all();
    let display_infos = DisplayInfo::all().unwrap();

    let mut ram = Ram{
        base_total_ram: sys.total_memory(),
        base_used_ram: sys.used_memory(),
        total_ram: sys.total_memory() as f64,
        used_ram: sys.used_memory() as f64,
        metric_prefix: String::from("bytes"),
        };

    // Uncomment if you want to change prefix to mb or gb
    //ram.to_mb();
    ram.set_to_gb();

    // Set output distance
    let distance = 5;

    let name = System::name().unwrap();
    let host = System::host_name().unwrap();
    let gpu = get_gpus()[0].clone();


    println!("System Name: {:>d$}", name, d = distance);
    println!("Host Name: {:>d$}", host, d = distance);
    println!("Memory: {:>d$.2} {} / {:>d$.2} {}", ram.used_ram(), ram.metric_prefix, ram.total_ram(), ram.metric_prefix, d=distance);
    println!("Processor: {}", sys.cpus()[0].brand());
    println!("GPUs: {}", gpu);
    println!("Monitor: {}", display_infos[0].friendly_name);
    println!("Display: {} x {} @ {}hz", display_infos[0].width, display_infos[0].height, display_infos[0].frequency);


    println!("Time: {:?}", start.elapsed());

}