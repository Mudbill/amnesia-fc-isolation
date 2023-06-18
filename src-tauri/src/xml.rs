use linked_hash_map::LinkedHashMap;
use std::{
    fs::{self, File},
    path::PathBuf,
};

use xml::{writer, EmitterConfig, EventReader};

// Config attributes and their default values

const DEFAULT_VALUES_CONFIG_FILES: [(&str, &str); 13] = [
    ("Resources", "resources.cfg"),
    ("Materials", "materials.cfg"),
    ("Game", "config/game.cfg"),
    ("Menu", "config/menu.cfg"),
    ("PreMenu", "config/pre_menu.cfg"),
    ("Demo", "config/demo.cfg"),
    ("DefaultMainSettings", "config/default_main_settings.cfg"),
    (
        "DefaultMainSettingsLow",
        "launcher/default_main_settings_low.cfg",
    ),
    (
        "DefaultMainSettingsMedium",
        "launcher/default_main_settings_medium.cfg",
    ),
    (
        "DefaultMainSettingsHigh",
        "launcher/default_main_settings_high.cfg",
    ),
    ("DefaultUserSettings", "config/default_user_settings.cfg"),
    ("DefaultUserKeys", "config/default_user_keys.cfg"),
    ("DefaultBaseLanguage", "config/base_english.cfg"),
];
const DEFAULT_VALUES_DIRECTORIES: [(&str, &str); 3] = [
    ("BaseLanguageFolder", "config/"),
    ("GameLanguageFolder", "config/lang/"),
    ("CustomStoryPath", "custom_stories"),
];
const DEFAULT_VALUES_START_MAP: [(&str, &str); 1] = [("Folder", "maps/main/")];

/// Mutate the main_init.cfg file for a mod,
/// by prepending all custom paths with the new mod location
pub fn mutate_main_init(file: &PathBuf, relative_path: &PathBuf) {
    let content = fs::read_to_string(file).expect("ERROR: Failed to read file");
    let outfile = File::create(file).unwrap();
    let reader = EventReader::from_str(&content);
    let mut writer = EmitterConfig::new()
        .write_document_declaration(false)
        .perform_indent(true)
        .create_writer(outfile);

    let prepend_value = String::from(relative_path.to_str().unwrap()) + "/";

    println!("INFO: Prepending '{}' to main init paths", prepend_value);

    for e in reader {
        let event = match e {
            Ok(e) => e,
            Err(e) => {
                dbg!(e);
                break;
            }
        };

        match event {
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => {
                // Create a write event with the current element's name
                let mut write_event = writer::XmlEvent::start_element(name.borrow());

                // Collect all attributes into a hash map
                let mut attribs: LinkedHashMap<String, String> = attributes
                    .iter()
                    .map(|a| (a.name.to_string(), a.value.to_owned()))
                    .collect();

                // Modify attributes
                if name.local_name == "ConfigFiles" {
                    for (key, default_value) in DEFAULT_VALUES_CONFIG_FILES {
                        edit_attrib(&mut attribs, key, default_value, &prepend_value);
                    }
                }

                if name.local_name == "Directories" {
                    for (key, default_value) in DEFAULT_VALUES_DIRECTORIES {
                        edit_attrib(&mut attribs, key, default_value, &prepend_value);
                    }
                }

                if name.local_name == "StartMap" {
                    for (key, default_value) in DEFAULT_VALUES_START_MAP {
                        edit_attrib(&mut attribs, key, default_value, &prepend_value);
                    }
                }

                // Attach each attribute to the write event
                for (name, value) in &attribs {
                    write_event = write_event.attr(name.as_str(), value.as_str());
                }

                // Write the whole element
                writer.write(write_event).unwrap();
            }
            xml::reader::XmlEvent::EndElement { name: _ } => {
                let write_event = writer::XmlEvent::end_element();
                writer.write(write_event).unwrap();
            }
            _ => (),
        }
    }
}

pub fn get_resources_path(file: &PathBuf) -> Option<PathBuf> {
    let content = fs::read_to_string(file).unwrap();
    let reader = EventReader::from_str(&content);
    for e in reader {
        let event = match e {
            Err(_) => {
                break;
            }
            Ok(e) => e,
        };
        match event {
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace: _namespace,
            } => {
                if name.local_name != "ConfigFiles" {
                    break;
                }

                match attributes.iter().find(|a| a.name.local_name == "Resources") {
                    None => {
                        println!("");
                        return Option::None;
                    }
                    Some(attrib) => return Option::Some(PathBuf::from(&attrib.value)),
                };
            }
            _ => (),
        }
    }
    return Option::None;
}

pub fn mutate_resources(file: &PathBuf, mod_path: &PathBuf) {
    let content = fs::read_to_string(file).unwrap();
    let outfile = File::create(file).unwrap();
    let reader = EventReader::from_str(&content);
    let mut writer = EmitterConfig::new()
        .write_document_declaration(false)
        .perform_indent(true)
        .create_writer(outfile);

    let mut is_in_directories = false;
    let mut has_added_resource = false;

    let resource_path = PathBuf::from("/").join(mod_path);
    let resource_path = resource_path.to_str().unwrap();

    for e in reader {
        match e.unwrap() {
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => {
                // Create a write event with the current element's name
                let mut write_event = writer::XmlEvent::start_element(name.borrow());

                // Collect all attributes into a hash map
                let attribs: LinkedHashMap<String, String> = attributes
                    .iter()
                    .map(|a| (a.name.to_string(), a.value.to_owned()))
                    .collect();

                if is_in_directories && !has_added_resource {
                    let extra_write_event = writer::XmlEvent::start_element("Directory")
                        .attr("Path", resource_path)
                        .attr("AddSubDirs", "true");
                    writer.write(extra_write_event).unwrap();
                    writer.write(writer::XmlEvent::end_element()).unwrap();
                    has_added_resource = true;
                    println!(
                        "INFO: Added new resource element with path '{}'",
                        resource_path
                    );
                }

                if name.local_name == "Resources" {
                    is_in_directories = true;
                }

                // Attach each attribute to the write event
                for (name, value) in &attribs {
                    write_event = write_event.attr(name.as_str(), value.as_str());
                }

                // Write the whole element
                writer.write(write_event).unwrap();
            }
            xml::reader::XmlEvent::EndElement { name: _ } => {
                let write_event = writer::XmlEvent::end_element();
                writer.write(write_event).unwrap();
            }
            _ => (),
        }
    }
}

fn edit_attrib(
    attribs: &mut LinkedHashMap<String, String>,
    key: &str,
    default_value: &str,
    prepend_value: &str,
) {
    let value = attribs.get_mut(key).unwrap();

    if value != default_value && !value.starts_with(prepend_value) {
        value.insert_str(0, prepend_value);
    }
}
