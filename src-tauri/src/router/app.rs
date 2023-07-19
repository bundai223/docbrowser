// router/app.rs

use crate::docset::{self, SearchIndex};

use super::{RouterBuilder};

pub(crate) fn mount() -> RouterBuilder {
	// getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
	<RouterBuilder>::new()
		.query("getAppName", |t| t(|_: (), _: ()| "rspc Test Project"))
		// .query("search", |t| t(|_, search_word: String| ok(search(&search_word))))
		.query("search", |t| {
            t(|_, search_word: String| { Ok::<String, rspc::Error>("Hello world".into()) })
            // t(|_, search_word: String|
            //     search(&search_word)
            // )
        })
}

#[tauri::command]
fn search(word: &str) -> Vec<SearchIndex> {
    let con = docset::open_my_db("./../docsets/hoge.db3").unwrap();
    let docsets = docset::search_docsets(&con, word);

    // let mut names: Vec<String> = Vec::new();
    // for d in docsets {
    //     names.push(d.name)
    // }

    let mut results: Vec<SearchIndex> = Vec::new();
    if word.is_empty() == false {
        let doc_con = docset::open_my_db("./../docsets/TypeScript.docset/Contents/Resources/docSet.dsidx").unwrap();
        let search_indices = docset::search_index(&doc_con, word);

        for index in search_indices {
            results.push(index)
        }
    }

    // names
    results
}

