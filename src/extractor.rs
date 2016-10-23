use std::fs;
use std::io;
use std::fs::File;
use std::path::{Component, PathBuf, Path};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use zip;

pub fn extract_zip(filename: &PathBuf, install_dir: &PathBuf) {
    let file = File::open(filename).expect("Could not open file.");
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = sanitize_filename(file.name());
        let install_path = Path::new(install_dir.as_path()).join(outpath);
        println!("{}", install_path.display());

        create_directory(install_path.parent().unwrap_or(Path::new("")), None);
        let perms = convert_permissions(file.unix_mode());

        if (&*file.name()).ends_with("/") {
            create_directory(&install_path, perms);
        } else {
            write_file(&mut file, &install_path, perms);
        }
    }
}

fn write_file(file: &mut zip::read::ZipFile, outpath: &Path, perms: Option<fs::Permissions>) {
    let mut outfile = File::create(&outpath).expect("Could not create a file in outpath.");
    io::copy(file, &mut outfile).expect("Could not copy.");
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).expect("Failed to set permissions.");
    }
}

fn create_directory(outpath: &Path, perms: Option<fs::Permissions>) {
    fs::create_dir_all(&outpath).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
}

#[cfg(unix)]
fn convert_permissions(mode: Option<u32>) -> Option<fs::Permissions> {
    match mode {
        Some(mode) => Some(fs::Permissions::from_mode(mode)),
        None => None,
    }
}

#[cfg(not(unix))]
fn convert_permissions(_mode: Option<u32>) -> Option<fs::Permissions> {
    None
}

fn sanitize_filename(filename: &str) -> PathBuf {
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    Path::new(no_null_filename)
        .components()
        .filter(|component| *component != Component::ParentDir)
        .fold(PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use tempdir::TempDir;

    #[test]
    fn test_extractor() {
        let pwd = env::current_dir().unwrap();
        let zipfile = pwd.join("fixture").join("test-extractor.zip");
        let extract_dir = TempDir::new("grnenv-rs").unwrap().into_path();
        extract_zip(&zipfile, &extract_dir);
        assert!(extract_dir.is_dir());
        assert!(extract_dir.join("test-extractor").is_dir());
        assert!(extract_dir.join("test-extractor").join("test.txt").exists());
        assert!(extract_dir.join("test-extractor").join("nested").is_dir());
        assert!(extract_dir.join("test-extractor").join("nested").join("test.txt").exists());
    }
}
