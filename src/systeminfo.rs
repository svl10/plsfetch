use sysinfo::System;

fn systeminfo() {
    let mut sys = System::new_all();
    sys.refresh_all();
}