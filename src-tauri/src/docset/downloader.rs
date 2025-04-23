use std::{env, fs};

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use tar::Archive;

pub async fn download_and_extract(
    url: &str,
    dest: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // let tmp_file = Path::new("./spec/tmp/tmp.tgz");
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let tmp_file = Path::new("./spec/tmp/tmp.tgz");
    match download_file(url, tmp_file).await {
        Ok(()) => println!("downloaded"),
        Err(why) => panic!("download {}: {}", url, why),
    };

    return extract(tmp_file, dest);
}

pub async fn download_file(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // let response = reqwest::blocking::get(url).unwrap();
    let response = reqwest::get(url).await?;
    // Ensure the parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(dest)?;
    copy(&mut response.bytes().await?.as_ref(), &mut file)?;

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
    println!("Extracting Done {:?}", dest);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[actix_rt::test]
    async fn test_download_docset() {
        let url = "http://sanfrancisco.kapeli.com/feeds/Rust.tgz";
        let dest = Path::new("./spec/tmp/extract/");
        let result = download_and_extract(url, &dest).await;

        assert!(result.is_ok()); // とりあえず結果を見る
    }
}
