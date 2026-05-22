

pub fn cat_stretch() -> (Vec<String>, usize){
    let thevec = vec![
        r#"_._     _,-'""`-._"#.to_string(),
        r#" (,-.`._,'(       |\`-/|"#.to_string(),
        r#"     `-.-' \ )-`( , o o)"#.to_string(),
        r#"           `-    \`_`"`-"#.to_string(),
    ];
    let width = thevec[3].len();
    (thevec, width)
}

pub fn cat_sit() -> (Vec<String>, usize){
    let thevec = vec![
        r"  /\_/\  ".to_string(),
        r" ( ^.^ ) ".to_string(),
        r"  )   (  ".to_string(),
        r" (     ) ".to_string(),
        r"  \___// ".to_string()
    ];

    let width = thevec[1].len();
    
    (thevec, width)
}

pub fn cat_curious() -> (Vec<String>, usize){
    let thevec = vec!{
        r" /\_/\".to_string(),
        r"( o.o )".to_string(),
        r" > ^ <".to_string()
    };

    let width = thevec[1].len();
    
    (thevec, width)
}

pub fn twob_heh() -> (Vec<String>, usize){
    let thevec = vec!{
r"⠄⠄⠄⠄⢠⣿⣿⣿⣿⣿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣯⢻⣿⣿⣿⣿⣆⠄⠄⠄".to_string(),
r"⠄⠄⣼⢀⣿⣿⣿⣿⣏⡏⠄⠹⣿⣿⣿⣿⣿⣿⣿⣿⣧⢻⣿⣿⣿⣿⡆⠄⠄".to_string(),
r"⠄⠄⡟⣼⣿⣿⣿⣿⣿⠄⠄⠄⠈⠻⣿⣿⣿⣿⣿⣿⣿⣇⢻⣿⣿⣿⣿⠄⠄".to_string(),
r"⠄⢰⠃⣿⣿⠿⣿⣿⣿⠄⠄⠄⠄⠄⠄⠙⠿⣿⣿⣿⣿⣿⠄⢿⣿⣿⣿⡄⠄".to_string(),
r"⠄⢸⢠⣿⣿⣧⡙⣿⣿⡆⠄⠄⠄⠄⠄⠄⠄⠈⠛⢿⣿⣿⡇⠸⣿⡿⣸⡇⠄".to_string(),
r"⠄⠈⡆⣿⣿⣿⣿⣦⡙⠳⠄⠄⠄⠄⠄⠄⢀⣠⣤⣀⣈⠙⠃⠄⠿⢇⣿⡇⠄".to_string(),
r"⠄⠄⡇⢿⣿⣿⣿⣿⡇⠄⠄⠄⠄⠄⣠⣶⣿⣿⣿⣿⣿⣿⣷⣆⡀⣼⣿⡇⠄".to_string(),
r"⠄⠄⢹⡘⣿⣿⣿⢿⣷⡀⠄⢀⣴⣾⣟⠉⠉⠉⠉⣽⣿⣿⣿⣿⠇⢹⣿⠃⠄".to_string(),
r"⠄⠄⠄⢷⡘⢿⣿⣎⢻⣷⠰⣿⣿⣿⣿⣦⣀⣀⣴⣿⣿⣿⠟⢫⡾⢸⡟⠄.".to_string(),
r"⠄⠄⠄⠄⠻⣦⡙⠿⣧⠙⢷⠙⠻⠿⢿⡿⠿⠿⠛⠋⠉⠄⠂⠘⠁⠞⠄⠄⠄".to_string(),
r"⠄⠄⠄⠄⠄⠈⠙⠑⣠⣤⣴⡖⠄⠿⣋⣉⣉⡁⠄⢾⣦⠄⠄⠄⠄⠄⠄⠄⠄".to_string()
    };

    let width = thevec[1].len();
    
    (thevec, width)
}