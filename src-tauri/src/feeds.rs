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
            Ok(e) => {
                match e {
                    XmlEvent::StartDocument { version, encoding, .. } => {
                        println!("StartDocument({version}, {encoding})")
                    },
                    XmlEvent::EndDocument => {
                        println!("EndDocument");
                        break;
                    }
                    XmlEvent::ProcessingInstruction { name, data } => {
                        println!("ProcessingInstruction({name}={:?})", data.as_deref().unwrap_or_default())
                    },
                    XmlEvent::StartElement { name, attributes, .. } => {
                        if attributes.is_empty() {
                            println!("StartElement({name})")
                        } else {
                            let attrs: Vec<_> = attributes
                                .iter()
                                .map(|a| format!("{}={:?}", &a.name, a.value))
                                .collect();
                            println!("StartElement({name} [{}])", attrs.join(", "))
                        }
                    }
                    XmlEvent::EndElement { name } => {
                        println!("EndElement({name})")
                    },
                    XmlEvent::Comment(data) => {
                        println!(r#"Comment("{}")"#, data.escape_debug())
                    }
                    XmlEvent::CData(data) => println!(r#"CData("{}")"#, data.escape_debug()),
                    XmlEvent::Characters(data) => {
                        println!(r#"Characters("{}")"#, data.escape_debug())
                    }
                    XmlEvent::Whitespace(data) => {
                        println!(r#"Whitespace("{}")"#, data.escape_debug())
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            },
        }
    }
    println!("Length: {}", depth);
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
    use super::*;

    #[test]
    fn test_read_feed() {
        let file = match File::open("./spec/Rust.xml") {
            Ok(it) => it,
            Err(why) => panic!("couldn't open {}: {}", "Rust.xml", why),
        };
        let bufreader = BufReader::new(file); // Buffering is important for performance
        read_feed(bufreader);
    }

    #[actix_rt::test]
    async fn test_buf_feed() {
        let content = match download_feed("https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml").await {
            Ok(it) => it,
            Err(why) => panic!("download {}: {}", "aaa", why),
        };
        read_feed(BufReader::new(content.as_bytes()));
    }
}