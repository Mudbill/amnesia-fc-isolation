use std::process::exit;

mod archive;
mod config;
mod fs;
mod utils;
mod xml;

fn main() {
    // Load required data about our environment
    let config = config::load_args();

    // Create common mods folder if not already existing
    utils::setup_mods_dir(&config.mods_dir);

    // Extract target mod into mods folder
    let did_install = archive::extract_archive(&config.src, &config.dst);

    // Locate where the main init file is
    let main_init = match fs::find_main_init(&config.dst) {
        None => {
            eprintln!("ERROR: Failed to locate main init file in extracted mod");
            exit(1);
        }
        Some(path) => {
            println!("INFO: Found main init file {:?}", path);
            path
        }
    };

    if did_install {
        // Change it, so that the paths are updated to reflect the mods folder location
        xml::mutate_main_init(&main_init, &config.relative_path);
    }

    // Find the (now mutated) path for the resources.cfg file
    let resources_path = match xml::get_resources_path(&main_init) {
        None => {
            eprintln!("ERROR: Couldn't find the Resources path inside main init file.");
            exit(1);
        }
        // Now join it with the game directory in order to find the file
        Some(path) => path,
    };

    if !resources_path.is_file() {
        eprintln!("ERROR: Resources file not found at {:?}", resources_path);
        exit(1);
    }

    println!("INFO: Found resources file at {:?}", resources_path);

    if did_install {
        // Change the resources file to add the mod's new directory at top priority
        xml::mutate_resources(&resources_path, &config.relative_path);
    }
}
