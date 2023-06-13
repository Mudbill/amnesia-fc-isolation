use std::{fs::File, path::Path, process::exit};

use archive::extract_archive;
use fs::get_basename;
use utils::relpath;

mod archive;
mod fs;
mod utils;
mod xml;

const GAME_DIR: &str = "./out/";
const MODS_DIR: &str = "./out/custom_stories/";
const MOD_TO_INSTALL: &str = "./assets/example_mod.rar";

fn main() {
    let mods_dir = String::from(MODS_DIR);
    utils::setup_mod_dir(&mods_dir);

    let mod_src = String::from(MOD_TO_INSTALL);
    let basename = get_basename(&mod_src);
    let mod_dst = Path::new(&mods_dir).join(basename);
    extract_archive(&mod_src, &mod_dst);

    let rel_mods_path = relpath(&String::from(MODS_DIR), &String::from(GAME_DIR));
    let rel_mods_path = rel_mods_path.join(basename);

    let main_init = fs::find_main_init(&String::from(mod_dst.to_str().unwrap()));

    match &main_init {
        None => {
            eprintln!("Failed to find main_init.cfg in extracted mod.");
            exit(-1);
        }
        Some(path) => {
            println!("Found main_init.cfg: {:?}", path);
        }
    }

    let main_init = main_init.unwrap();
    let main_init_content = fs::read_file(&main_init.as_path());
    let main_init_file = File::create(&main_init).unwrap();

    let prepend_path = rel_mods_path.to_str().unwrap().to_owned() + "/";

    xml::mutate_main_init(main_init_content, &main_init_file, prepend_path.as_str());

    let main_init_content = fs::read_file(&main_init.as_path());

    let prepend_path = String::from("/") + &prepend_path;

    let resources_path = xml::get_resources(&main_init_content).unwrap();
    let resources_path = String::from(GAME_DIR) + &resources_path;
    let resources_content = fs::read_file(Path::new(&resources_path));
    let resources_file = File::create(Path::new(&resources_path)).unwrap();
    xml::mutate_resources(resources_content, resources_file, &prepend_path);
}
