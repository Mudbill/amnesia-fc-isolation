use std::{
    fs::{self},
    path::{Path, PathBuf},
};

pub fn find_main_init(mod_dir: &String) -> Option<PathBuf> {
    // First we try the most obvious location
    let test_path = Path::new(mod_dir).join("config").join("main_init.cfg");
    if test_path.is_file() {
        return Option::Some(test_path);
    }
    // If that isn't found, let's check the whole directory for a config folder
    let paths = fs::read_dir(mod_dir).unwrap();
    for path in paths {
        if path
            .as_ref()
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .contains("config")
        {
            if path.as_ref().unwrap().path().is_dir() {
                // And in that config folder, let's check for a main_init.cfg file
                let paths = fs::read_dir(path.unwrap().path()).unwrap();
                for path in paths {
                    if path
                        .as_ref()
                        .unwrap()
                        .path()
                        .to_str()
                        .unwrap()
                        .contains("main_init")
                    {
                        if path.as_ref().unwrap().path().is_file() {
                            return Option::Some(path.unwrap().path());
                        }
                    }
                }
            }
        }
    }
    return Option::None;
}

pub fn get_basename(path: &String) -> &str {
    Path::new(path).file_stem().unwrap().to_str().unwrap()
}

pub fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}
