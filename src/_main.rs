use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

use crate::archive::extract_rar;
use crate::archive::extract_zip;

pub mod archive;

const MOD_DIR: &str = "/Users/magnusb/Desktop/mods";

fn main() {
    let mod_dir = String::from(MOD_DIR);
    setup_mod_dir(&mod_dir);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file path given");
        exit(-1);
    }

    let path = &args[1];

    let target = Path::new(&path);
    if !target.is_file() {
        println!("File not found: {}", path);
        exit(-1);
    }

    let path = String::from(path);

    let basename = target.file_stem().unwrap().to_str().unwrap();
    let ext = target.extension().unwrap().to_str().unwrap();

    let outpath = &Path::new(&mod_dir).join(basename);

    println!(
        "Installing {} into {}",
        &target.to_str().unwrap(),
        &outpath.to_str().unwrap()
    );

    if !outpath.exists() {
        match ext {
            "rar" => {
                println!("Extracting rar file");
                extract_rar(path, String::from(outpath.to_str().unwrap()));
            }
            "zip" => {
                println!("Extracting zip file");
                extract_zip(path, outpath);
            }
            _ => {
                println!("Unsupported archive format");
                exit(-1);
            }
        }
    } else {
        println!("Mod already installed, continuing");
    }

    let main_init = find_main_init(&String::from(outpath.to_str().unwrap()));
    if main_init != "" {
        println!("Found main_init: {}", main_init);
    } else {
        println!("Could not find main_init");
        exit(-1);
    }

    let contents = fs::read_to_string(main_init).expect("Failed to read main_init.cfg file");
    parse_xml(contents);
}

fn parse_xml(contents: String) {
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Text(e)) => {
                println!("{}", e.unescape().unwrap().into_owned());
            }
            Ok(Event::Start(e)) => {
                println!("{:?}", e.name());
            }
            Ok(Event::End(e)) => {
                println!("{:?}", e.name());
            }
            Ok(Event::Empty(e)) => {}
            Ok(Event::Comment(e)) => {}
            Ok(Event::CData(e)) => {}
            Ok(Event::Decl(e)) => {}
            Ok(Event::PI(e)) => {}
            Ok(Event::DocType(e)) => {}
        }
    }
}

fn setup_mod_dir(path: &String) {
    if !exists(path) {
        fs::create_dir_all(path).expect("Failed to set up mod directory!");
    }
}

fn exists(path: &String) -> bool {
    return Path::new(path).exists();
}

fn find_main_init(mod_dir: &String) -> String {
    // First we try the most obvious location
    let test_path = Path::new(mod_dir).join("config").join("main_init.cfg");
    if test_path.is_file() {
        return String::from(test_path.to_str().unwrap());
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
                            return String::from(path.unwrap().path().to_str().unwrap());
                        }
                    }
                }
            }
        }
    }
    return String::new();
}
