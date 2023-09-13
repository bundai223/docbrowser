// router/app.rs

use rspc::Type;

use crate::docset::{self, SearchIndex};

use super::{RouterBuilder};

#[derive(Type, serde::Serialize)]
struct SearchResult {
    indices: Vec<SearchIndex>
}

pub(crate) fn mount() -> RouterBuilder {
	// getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
	<RouterBuilder>::new()
		.query("getAppName", |t| t(|_: (), _: ()| "rspc Test Project"))
		// .query("search", |t| t(|_, search_word: String| ok(search(&search_word))))
		.query("search", |t| {
            // t(|_, search_word: String| { Ok::<String, rspc::Error>("Hello world".into()) })
            t(|_, search_word: String|
                search(&search_word)
            )
        }
    )
}

#[tauri::command]
fn search(word: &str) -> SearchResult {
    if word.is_empty() {
        return SearchResult { indices: Vec::new() };
    }

    let target_docset_name = "TypeScript";
    let docsets_connection = docset::open_my_db(&docset::docsets_mater_db_path()).unwrap();
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

