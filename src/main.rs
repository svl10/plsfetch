use sysinfo::{
    Components, Disks, Networks, System,
};

// use ocl::{Platform, Device};
struct Ram {
    base_total_ram: u64,
    base_used_ram: u64,
    total_ram: f64,
    used_ram: f64,
    metric_prefix: String,
}

impl Default for Ram {
    fn default() -> Self {
        Ram{
            base_total_ram: 0,
            base_used_ram: 0,
            total_ram: 0.0,
            used_ram: 0.0,
            metric_prefix: String::from("bytes")

        }
    }
}

impl Ram {
    fn new(base_total_ram: u64, base_used_ram: u64, total_ram: f64, used_ram: f64) -> Ram {
        Ram { base_total_ram, base_used_ram, total_ram, used_ram, metric_prefix: String::new()}
    }

    fn total_ram(&self) -> f64 {
        self.total_ram
    }
    fn used_ram(&self) -> f64 {
        self.used_ram
    }
    fn metric_prefix(&self) -> String {
        self.metric_prefix.clone()
    }

    fn to_mb(&mut self) {
        self.total_ram = self.base_total_ram as f64 / 1024.0 / 1024.0;
        self.used_ram = self.base_used_ram as f64 / 1024.0 / 1024.0;
        self.metric_prefix = String::from("MB");
    }

    fn to_gb(&mut self) {
        self.total_ram = self.base_total_ram as f64 / 1024.0 / 1024.0 / 1024.0;
        self.used_ram = self.base_used_ram as f64 / 1024.0 / 1024.0 / 1024.0;
        self.metric_prefix = String::from("GB");
    }


}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    // let platform = Platform::default(); // Pick the default platform
    // let devices = platform.vendor().unwrap();
    // //let devices = platform.devices(ocl::flags::DEVICE_TYPE_GPU).unwrap();
    // println!("Available devices: {:?}", devices);

    let mut ram = Ram{
        base_total_ram: sys.total_memory(),
        base_used_ram: sys.used_memory(),
        total_ram: sys.total_memory() as f64,
        used_ram: sys.used_memory() as f64,
        metric_prefix: String::from("bytes"),
        };

    // Uncomment if you want to change prefix to mb or gb
    //ram.to_mb();
    ram.to_gb();

    // Set output distance
    let distance = 5;

    let name = System::name().unwrap();
    let host = System::host_name().unwrap();


    println!("System Name: {:>d$}", name, d = distance);
    println!("Host Name: {:>d$}", host, d = distance);
    println!("Memory: {:>d$.2} {}  / {:>d$.2} {}", ram.used_ram(), ram.metric_prefix, ram.total_ram(), ram.metric_prefix, d=distance);
    println!("Processor: {}", sys.cpus()[0].brand());
}