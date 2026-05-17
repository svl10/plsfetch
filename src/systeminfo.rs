use sysinfo::System;


pub fn systeminfo() -> Sysinfo{
    let mut sys = System::new_all();
    sys.refresh_all();

    let sys_name= System::name().unwrap();
    let host_name = System::host_name().unwrap();
    let cpu_name = sys.cpus()[0].brand().to_string();
    let used_ram = sys.used_memory();
    let total_ram = sys.total_memory();

    Sysinfo { sys_name, host_name, cpu_name, used_ram, total_ram }
}

#[derive(Debug)]
pub struct Sysinfo {
    sys_name: String,
    host_name: String,
    cpu_name: String,
    used_ram: u64,
    total_ram: u64
}

impl Sysinfo{
    pub fn sys_name(&self) -> String{
        self.sys_name.clone()
    }

    pub fn host_name(&self) -> String{
        self.host_name.clone()
    }

    pub fn cpu_name(&self) -> String{
        self.cpu_name.clone()
    }

    pub fn used_ram(&self) -> f64{
        self.used_ram as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn total_ram(&self) -> f64{
        self.total_ram as f64 / 1024.0 / 1024.0 / 1024.0
    }
}