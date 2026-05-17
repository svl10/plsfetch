use display_info::DisplayInfo;

pub fn displayinfo() -> String{
    let display_infos = DisplayInfo::all().unwrap();
    let theinfo = String::from(format!("Display: {} x {} @ {}", display_infos[0].width, display_infos[0].height, display_infos[0].frequency));
    theinfo
}