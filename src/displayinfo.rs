use display_info::DisplayInfo;

pub fn displayinfo() -> (String, String){
    let display_infos = DisplayInfo::all().unwrap();
    let theinfo = String::from(format!("Display: {} x {} @ {}", display_infos[0].width, display_infos[0].height, display_infos[0].frequency));
    let monitor = String::from(format!("Monitor: {}", display_infos[0].friendly_name));
    
    (theinfo, monitor)
}