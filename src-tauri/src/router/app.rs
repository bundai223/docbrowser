// router/app.rs

use rspc::Type;

use crate::docset::{self, SearchIndex};

use super::RouterBuilder;

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

fn search(word: &str) -> SearchResult {
    let mut result = SearchResult { indices: Vec::new() };
    // let mut results: Vec<SearchIndex> = Vec::new();
    if word.is_empty() == false {
        let docset_name = "TypeScript";
        let docset_base_path = "./../docsets";
        let sqlite_file_path = "Contents/Resources/docSet.dsidx";
        let docset_path = format!("{}/{}.docset/{}", docset_base_path, docset_name, sqlite_file_path);

        let doc_con = docset::open_my_db(&docset_path).unwrap();
        let search_indices = docset::search_index(&doc_con, docset_name, word);

        for index in search_indices {
            result.indices.push(index)
        }
    }

    // names
    result
}

