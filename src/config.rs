use std::{env, path::PathBuf, process::exit};

use crate::utils::relpath;

pub struct Config {
    pub game_dir: PathBuf,
    pub mods_dir: PathBuf,
    pub src: PathBuf,
    pub dst: PathBuf,
    pub relative_path: PathBuf,
}

pub fn load_args() -> Config {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("ERROR: Missing argument for target mod");
        exit(1);
    }

    let cwd = env::current_dir().unwrap();
    let mods_dir = cwd.join("custom_stories");
    let src = PathBuf::from(&args[1]);

    if !src.is_file() && !src.is_dir() {
        eprintln!("ERROR: Target not found: {:?}", src);
        exit(2);
    }

    println!("INFO: Found target {:?}", src.canonicalize().unwrap());

    let basename = src.file_stem().unwrap().to_str().unwrap();
    let dst = mods_dir.join(basename);

    let relative_path = relpath(&dst, &cwd);

    let config = Config {
        game_dir: cwd,
        mods_dir,
        src,
        dst,
        relative_path,
    };

    return config;
}
