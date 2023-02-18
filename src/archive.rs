use std::fs;
use std::io;
use std::path::Path;
use unrar::Archive;

pub fn extract_rar(file_path: String, out_path: String) {
    Archive::new(file_path)
        .extract_to(String::from(out_path))
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
