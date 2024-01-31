// router/app.rs


use std::path::Path;

use rspc::Type;

use crate::docset::{self, Docset, SearchIndex};
// use crate::feeds::docset_url_from_feed;
use crate::docsetDownloader::download_and_extract;

use super::{RouterBuilder};

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
		// .query("download_docset", |t| {
        //     t(|_, docset_name: String|
        //         download_docset(&docset_name)
        //     )
        // })
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

fn download_docset(word: &str) {
    let dest = Path::new("Rust.tgz");
    download_and_extract(word, dest);
}
