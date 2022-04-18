use std::{env, fs, io::Write, path};

pub fn initialize() -> std::io::Result<()> {
    let home = path::PathBuf::from(env::var("HOME").unwrap());
    let dir_path = home.join(".pm");
    let db_path = dir_path.join("db.json");
    let display = &db_path.display();
    if dir_path.exists() {
        fs::remove_dir(&dir_path).ok();
    };
    if !&db_path.exists() {
        let _dir = fs::create_dir(dir_path)?;
        let mut file = match fs::File::create(&db_path) {
            Err(reason) => panic!("Could not create {} : {}", display, reason),
            Ok(file) => file,
        };

        match file.write_all("[]".as_bytes()) {
            Err(reason) => panic!("Could not create {} : {}", display, reason),
            Ok(_) => {
                println!("Project Manager initialized. Run --help to learn more.")
            }
        }
        // match dir {
        //     Ok(()) => {
        //         let mut file = match fs::File::create(&db_path) {
        //             Err(reason) => panic!("Could not create {} : {}", display, reason),
        //             Ok(file) => file,
        //         };

        //         match file.write_all("[]".as_bytes()) {
        //             Err(reason) => panic!("Could not create {} : {}", display, reason),
        //             Ok(_) => {
        //                 println!("Project Manager initialized. Run --help to learn more.")
        //             }
        //         }
        //     }
        //     Err(err) => {
        //         panic!("Something went wrong while Initialize process : {}", err);
        //     }
        //};
    } else {
        println!("Project Manager is already initialized.");
    }
    Ok(())
}
