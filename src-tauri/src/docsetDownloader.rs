use std::fs;

use std::fs::File;
use std::io::copy;
use std::path::Path;
use tar::Archive;
use flate2::read::GzDecoder;

pub fn download_and_extract(url: &str, dest: &Path) {
    // let tmp_file = Path::new("./spec/tmp/tmp.tgz");
    let tmp_file = Path::new("./spec/tmp/tmp.tgz");
    match download_file(url, tmp_file) {
        Ok(()) => println!("downloaded"),
        Err(why) => panic!("download {}: {}", url, why),
    };

    extract(tmp_file, dest);
}

pub fn download_file(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>>{
    let response = reqwest::blocking::get(url).unwrap();
    let mut file = File::create(dest).unwrap();
    copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();

    Ok(())
}

pub fn extract(tgz_path: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let tar_gz = File::open(tgz_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    println!("Extracting {:?} to {:?}", tgz_path, dest);

    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    archive.unpack(dest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_download_docset() {
        let url = "http://sanfrancisco.kapeli.com/feeds/Rust.tgz";
        let dest = Path::new("./spec/tmp/extract/");
        download_and_extract(url, &dest);
    }
}
