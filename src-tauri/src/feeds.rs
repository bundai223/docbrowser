// 参考: https://github.com/kapeli/feeds
// https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml

pub fn docset_url_from_feed(xml_string: &str) -> String {
    let doc = match roxmltree::Document::parse(xml_string) {
        Ok(it) => it,
        Err(why) => panic!("couldn't open {}: {}", "Rust.xml", why),
    };

    // let doc = roxmltree::Document::parse(xml_string)?;
    let elem = match doc.descendants().find(|n| n.has_tag_name("url")) {
        None => panic!("noting"),
        Some(e) => e,
    };
    // return Ok(String::from("suc"));
    return String::from(elem.text().unwrap());
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
        let size = match file.read_to_string(&mut xml_string) {
            Ok(it) => it,
            Err(why) => panic!("{}", why),
        };
        println!("{} bytes, {}", size, xml_string);
        let url = docset_url_from_feed(&xml_string);
        assert!(url == "http://sanfrancisco.kapeli.com/feeds/Rust.tgz");
    }

    #[actix_rt::test]
    async fn test_buf_feed() {
        let feed_url = "https://raw.githubusercontent.com/Kapeli/feeds/mastera/Rust.xml";

        let content = match download_feed(feed_url).await {
            Ok(it) => it,
            Err(why) => panic!("download {}: {}", feed_url, why),
        };
        let url = docset_url_from_feed(&content);
        assert!(url == "http://sanfrancisco.kapeli.com/feeds/Rust.tgz");
    }
}
