// router/app.rs


use std::path::Path;

use rspc::Type;

use crate::docset::{self, docset_path, Docset, SearchIndex};
// use crate::feeds::docset_url_from_feed;
use crate::docset_downloader::download_and_extract;
use crate::feeds::{docset_url_from_feed, download_feed};

use super::RouterBuilder;

#[derive(Type, serde::Serialize)]
struct SearchResult {
    indices: Vec<SearchIndex>
}

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new()
	    // getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
		.query("getAppName", |t| t(|_: (), _: ()| "rspc Test Project"))
		.query("search", |t| {
            // t(|_, search_word: String| { Ok::<String, rspc::Error>("Hello world".into()) })
            t(|_, search_word: String|
                search(&search_word)
            )
        })
		.query("docsets", |t| t(|_: (), _: ()| docsets()))
		.mutation("download_docset", |t| {
            t(|_, to_download: ToDownloadDocset| async {
                download_docset(to_download).await;
            })
        })
}

fn docsets() -> Vec<Docset> {
    let docsets_connection = docset::open_my_db(&docset::docsets_master_db_path()).unwrap();
    // let docsets = docset::search_docsets(&docsets_connection, word);
    let docsets = docset::docsets(&docsets_connection);

    docsets
}

fn search(word: &str) -> SearchResult {
    if word.is_empty() {
        return SearchResult { indices: Vec::new() };
    }

    let target_docset_name = "TypeScript";
    let docsets_connection = docset::open_my_db(&docset::docsets_master_db_path()).unwrap();
    // let docsets = docset::search_docsets(&docsets_connection, word);
    let docsets = docset::search_docsets(&docsets_connection, target_docset_name);

    let mut result = SearchResult { indices: Vec::new() };

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
    feed_url: String
}

async fn download_docset(to_download_docset: ToDownloadDocset) -> Result<(), ()> {
    println!("{}, {}", to_download_docset.name, to_download_docset.feed_url);

    let content = match download_feed(&(to_download_docset.feed_url)).await {
        Ok(it) => it,
        Err(why) => panic!("download {}: {}", to_download_docset.feed_url, why),
    };
    let url = docset_url_from_feed(&content);

    let dest_path: String = docset_path(to_download_docset.name);
    let dest = Path::new(&dest_path);
    download_and_extract(&url, dest).await;

    Ok(())
}
