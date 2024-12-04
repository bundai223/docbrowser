// router/app.rs

use std::path::Path;

use specta::Type;

use crate::docset::{self, docsets_base_path, Docset, SearchIndex};
// use crate::feeds::docset_url_from_feed;
use crate::docset_downloader::download_and_extract;
use crate::feeds::{docset_url_from_feed, download_feed};

use super::RouterBuilder;

#[derive(Type, serde::Serialize)]
struct SearchResult {
    indices: Vec<SearchIndex>,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        // getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
        .query("getAppName", |t| t(|_: (), _: ()| "rspc Test Project"))
        .query("search", |t| {
            // t(|_, search_word: String| { Ok::<String, rspc::Error>("Hello world".into()) })
            t(|_, search_word: String| search(&search_word))
        })
        .query("docsets", |t| t(|_: (), _: ()| docsets()))
        .mutation("download_docset", |t| {
            t(|_, to_download: ToDownloadDocset| async {
                let docset_name = to_download.name.clone();
                match download_docset(to_download).await {
                    Ok(it) => it,
                    Err(_) => panic!("download docset rspc handler: {}", docset_name),
                }
            })
        })
}

fn docsets() -> Vec<Docset> {
    let docsets_connection = docset::open_my_db(&docset::docsets_master_db_path()).unwrap();
    // let docsets = docset::search_docsets(&docsets_connection, word);
    let docsets = {
        let con: &Connection = &docsets_connection;
        // let mut stmt = con.prepare("select id, name, alias, feed_url, docset_path, downloaded from docsets").unwrap();
        let mut stmt = con.prepare("select * from docsets").unwrap();
        let docset_results = stmt
            .query_map(params![], |row| {
                Ok(Docset {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    alias: row.get(2).unwrap(),
                    feed_url: row.get(3).unwrap(),
                    docset_path: row.get(4).unwrap(),
                    downloaded: row.get(5).unwrap(),
                })
            })
            .unwrap();

        let mut _docsets: Vec<Docset> = Vec::new();
        for d in docset_results {
            let docset = d.unwrap();
            // println!("{:?}", docset);
            _docsets.push(docset);
        }
        _docsets
    };

    docsets
}

fn search(word: &str) -> SearchResult {
    if word.is_empty() {
        return SearchResult {
            indices: Vec::new(),
        };
    }

    let target_docset_name = "TypeScript";
    let docsets_connection = docset::open_my_db(&docset::docsets_master_db_path()).unwrap();
    // let docsets = docset::search_docsets(&docsets_connection, word);
    let docsets = docset::search_docsets(&docsets_connection, target_docset_name);

    let mut result = SearchResult {
        indices: Vec::new(),
    };

    for d in docsets {
        // let doc_con = docset::open_my_db("./../docsets/TypeScript.docset/Contents/Resources/docSet.dsidx").unwrap();
        let doc_con = docset::open_my_db(&docset::docset_db_path(target_docset_name)).unwrap();
        let search_indices = docset::search_index(&doc_con, word, d);

        for index in search_indices {
            result.indices.push(index)
        }
    }

    result
}

#[derive(Type, serde::Deserialize)]
struct ToDownloadDocset {
    name: String,
    feed_url: String,
}

async fn download_docset(to_download_docset: ToDownloadDocset) -> Result<(), ()> {
    println!(
        "{}, {}",
        to_download_docset.name, to_download_docset.feed_url
    );

    let content = match download_feed(&(to_download_docset.feed_url)).await {
        Ok(it) => it,
        Err(why) => panic!("download feed {}: {}", to_download_docset.feed_url, why),
    };
    let url = docset_url_from_feed(&content);

    // let dest_path: String = docset_path(to_download_docset.name);
    // let dest = Path::new(&dest_path);
    let dest_path_str = docsets_base_path();
    let dest = Path::new(&dest_path_str);

    match download_and_extract(&url, dest).await {
        Ok(it) => it,
        Err(why) => panic!("download docset {}: {}", url, why),
    }

    Ok(())
}
