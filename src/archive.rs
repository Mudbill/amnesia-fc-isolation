use std::fs;
use std::io;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use unrar::Archive;

pub fn extract_archive(src: &PathBuf, dst: &PathBuf) -> bool {
    // If destination already exists, the mod is probably already installed, so abort
    if dst.exists() {
        println!("INFO: Destination is already in use, skipping install.");
        return false;
    }

    // If target is a folder, just copy the folder
    if src.is_dir() {
        println!("INFO: Copying folder from {:?} to {:?}", src, dst);
        extract_folder(src, dst).unwrap();
        return true;
    }

    // If target is a file, make sure it's found
    if !src.is_file() {
        println!("File not found: {:?}", src);
        exit(-1);
    }

    let ext = src.extension().unwrap().to_str().unwrap();

    match ext {
        "rar" => {
            println!("INFO: Extracting RAR archive {:?} to {:?}", src, dst);
            extract_rar(
                src.to_str().unwrap().to_string(),
                String::from(dst.to_str().unwrap()),
            );
            return true;
        }
        "zip" => {
            println!("INFO: Extracting ZIP archive {:?} to {:?}", src, dst);
            extract_zip(src.to_str().unwrap().to_string(), &dst);
            return true;
        }
        _ => {
            eprintln!("ERROR: Unsupported archive format extension {}", ext);
            return false;
        }
    }
}

fn extract_folder(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            extract_folder(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn extract_rar(file_path: String, out_path: String) {
    Archive::new(file_path)
        .extract_to(out_path)
        .expect("Failed to extract")
        .process()
        .expect("Failed to extract");
}

pub fn extract_zip(file_path: String, out_path: &Path) {
    let file = fs::File::open(file_path).expect("Error");
    let mut archive = zip::ZipArchive::new(file).expect("Error");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Error");
        let outpath = match file.enclosed_name() {
            Some(path) => out_path.join(path).to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).expect("Error");
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).expect("Error");
                }
            }
            let mut outfile = fs::File::create(&outpath).expect("Error");
            io::copy(&mut file, &mut outfile).expect("Error");
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).expect("Error");
            }
        }
    }
}
