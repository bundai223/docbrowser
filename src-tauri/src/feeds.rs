// 参考: https://github.com/kapeli/feeds
// https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml

use std::io::BufReader;

fn read_feed(xml_string: &str) {
let doc = roxmltree::Document::parse(xml_string);
}

// async fn download_feed() -> Result<(), reqwest::Error> {
async fn download_feed(url: &str) -> Result<String, reqwest::Error> {
let content = reqwest::get(url)
    .await?
    .text()
    .await?;
Ok(content)
}



#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_read_feed() {
        let mut file = match File::open("./spec/Rust.xml") {
            Ok(it) => it,
            Err(why) => panic!("couldn't open {}: {}", "Rust.xml", why),
        };
        let mut xml_string: String = String::new();
        file.read_to_string(&mut xml_string);
        read_feed(&xml_string);
    }

    #[actix_rt::test]
    async fn test_buf_feed() {
        let feed_url = "https://raw.githubusercontent.com/Kapeli/feeds/mastera/Rust.xml";

        let content = match download_feed(feed_url).await {
            Ok(it) => it,
            Err(why) => panic!("download {}: {}", feed_url, why),
        };
        read_feed(&content);
    }
}
