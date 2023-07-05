// 参考: https://github.com/kapeli/feeds
// https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml

use std::fs::File;
use std::io::{BufReader};

use xml::reader::{EventReader, XmlEvent};

fn read_feed<R: std::io::Read>(bufreader: BufReader<R>) {
    let parser = EventReader::new(bufreader);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    println!("Length: {}", depth);
}

// async fn download_feed() -> Result<(), reqwest::Error> {
async fn download_feed() -> Result<String, reqwest::Error> {
    let content = reqwest::get("https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml")
        .await?
        .text()
        .await?;
    Ok(content)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_feed() {
        let file = match File::open("Rust.xml") {
            Ok(it) => it,
            Err(why) => panic!("couldn't open {}: {}", "Rust.xml", why),
        };
        let bufreader = BufReader::new(file); // Buffering is important for performance
        read_feed(bufreader);
    }

    #[actix_rt::test]
    async fn test_buf_feed() {
        let content = match download_feed().await {
            Ok(it) => it,
            Err(why) => panic!("download {}: {}", "aaa", why),
        };
        read_feed(BufReader::new(content.as_bytes()));
    }
}