// router/app.rs

use rspc::Type;

use crate::docset::{self, SearchIndex};

use super::RouterBuilder;

#[derive(Type, serde::Serialize)]
struct SearchResult {
    indices: Vec<SearchIndex>
}

#[derive(Type, serde::Deserialize)]
struct DocPage {
    pub docset_name: String,
    pub rel_path: String,
}

pub(crate) fn mount() -> RouterBuilder {
	// getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
	<RouterBuilder>::new()
		.query("getAppName", |t| t(|_: (), _: ()| "DocBrowser"))
		// .query("search", |t| t(|_, search_word: String| ok(search(&search_word))))
		.query("search", |t| {
            // t(|_, search_word: String| { Ok::<String, rspc::Error>("Hello world".into()) })
            t(|_, search_word: String|
                search(&search_word)
            )
        })
        .query("read_html", |t| {
            t(|_, page_info: DocPage|
                read_html(&page_info.docset_name, &page_info.rel_path)
            )
        })
}

fn search(word: &str) -> SearchResult {
    let mut result = SearchResult { indices: Vec::new() };
    // let mut results: Vec<SearchIndex> = Vec::new();

    if word.is_empty() == false {
        let docset_name = "TypeScript";
        let docset_path = docset::docset_database_path(docset_name);

        let doc_con = docset::open_my_db(&docset_path).unwrap();
        let search_indices = docset::search_index(&doc_con, docset_name, word);

        for index in search_indices {
            result.indices.push(index)
        }
    }

    // names
    result
}

fn read_html(docset_name: &str, rel_path: &str) -> String {
    // let dummy_str = format!("Hello dummy: {}", path);
    // return dummy_str.to_string();
    let html = docset::read_html(docset_name, rel_path);

    return html;
}