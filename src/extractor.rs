use flate2::read::GzDecoder;
use std::fs;
use std::fs::File;
use std::io;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path, PathBuf};
use tar::Archive;
use zip;

pub fn extract_zip(filename: &PathBuf, install_dir: &PathBuf) -> Result<(), io::Error> {
    let file = File::open(filename)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = sanitize_filename(file.name());
        let install_path = Path::new(install_dir.as_path()).join(outpath);
        println!("{}", install_path.display());

        create_directory(install_path.parent().unwrap_or(Path::new("")), None)?;
        let perms = convert_permissions(file.unix_mode());

        if (&*file.name()).ends_with("/") {
            create_directory(&install_path, perms)?;
        } else {
            write_file(&mut file, &install_path, perms)?;
        }
    }
    Ok(())
}

pub fn extract_targz(targz: &PathBuf, install_dir: &PathBuf) -> Result<(), io::Error> {
    let tarball = File::open(targz)?;
    let gz = GzDecoder::new(tarball);
    let mut tar = Archive::new(gz);
    tar.unpack(install_dir)?;
    Ok(())
}

fn write_file(
    file: &mut zip::read::ZipFile,
    outpath: &Path,
    perms: Option<fs::Permissions>,
) -> Result<(), io::Error> {
    let mut outfile = File::create(&outpath)?;
    io::copy(file, &mut outfile)?;
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms)?;
    }
    Ok(())
}

fn create_directory(outpath: &Path, perms: Option<fs::Permissions>) -> Result<(), io::Error> {
    fs::create_dir_all(&outpath)?;
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms)?;
    }
    Ok(())
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
    fn test_zip_extractor() {
        let pwd = env::current_dir().unwrap();
        let zipfile = pwd.join("fixture").join("test-extractor.zip");
        let extract_dir = TempDir::new("grnenv-rs").unwrap().into_path();
        assert!(extract_zip(&zipfile, &extract_dir).is_ok());
        assert!(extract_dir.is_dir());
        assert!(extract_dir.join("test-extractor").is_dir());
        assert!(extract_dir.join("test-extractor").join("test.txt").exists());
        assert!(extract_dir.join("test-extractor").join("nested").is_dir());
        assert!(
            extract_dir
                .join("test-extractor")
                .join("nested")
                .join("test.txt")
                .exists()
        );
    }

    #[test]
    fn test_targz_extractor() {
        let pwd = env::current_dir().unwrap();
        let targz = pwd.join("fixture").join("test-extractor.tar.gz");
        let extract_dir = TempDir::new("grnenv-rs").unwrap().into_path();
        assert!(extract_targz(&targz, &extract_dir).is_ok());
        assert!(extract_dir.is_dir());
        assert!(extract_dir.join("test-extractor").is_dir());
        assert!(extract_dir.join("test-extractor").join("test.txt").exists());
        assert!(extract_dir.join("test-extractor").join("nested").is_dir());
        assert!(
            extract_dir
                .join("test-extractor")
                .join("nested")
                .join("test.txt")
                .exists()
        );
    }
}
