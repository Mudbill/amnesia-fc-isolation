use std::{
    fs,
    path::{Path, PathBuf},
};

use pathdiff::diff_paths;

/// Get a relative path from a to b, if possible. Otherwise returns a.
pub fn relpath(a: &PathBuf, b: &PathBuf) -> PathBuf {
    let rel = diff_paths(a, b);
    match rel {
        None => Path::new(a).to_path_buf(),
        Some(val) => val,
    }
}

/// Create all directories for the mod dir
pub fn setup_mods_dir(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to set up mod directory!");
    }
}
