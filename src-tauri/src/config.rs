use std::{env, path::PathBuf, process::exit};

use crate::utils::relpath;

pub struct Config {
    pub game_dir: PathBuf,
    pub mods_dir: PathBuf,
    pub src: PathBuf,
    pub dst: PathBuf,
    pub relative_path: PathBuf,
}

pub fn load_args(game_dir: String, target_mod: String) -> Config {
    let game_dir = PathBuf::from(game_dir);
    let mods_dir = game_dir.join("custom_stories");
    let src = PathBuf::from(target_mod);

    if !src.is_file() && !src.is_dir() {
        eprintln!("ERROR: Target not found: {:?}", src);
        exit(2);
    }

    println!("INFO: Found target {:?}", src.canonicalize().unwrap());

    let basename = src.file_stem().unwrap().to_str().unwrap();
    let dst = mods_dir.join(basename);

    let relative_path = relpath(&dst, &game_dir);

    let config = Config {
        game_dir,
        mods_dir,
        src,
        dst,
        relative_path,
    };

    return config;
}
