// router/app.rs

use std::path::Path;

use rusqlite::params;
use rusqlite::Connection;
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
            // Add context argument `_ctx`
            t(|_ctx, search_word: String| search(&search_word))
        })
        // Add context argument `_ctx`
        .query("docsets", |t| t(|_ctx, _: ()| docsets()))
        .mutation("download_docset", |t| {
             // Add context argument `_ctx` and remove unnecessary async block
            t(|_ctx, to_download: ToDownloadDocset| download_docset(to_download))
        })
}

// Add rspc::Error to the imports if not already present (it should be implicitly available via RouterBuilder)
// use rspc::Error; // Might be needed explicitly depending on context

fn docsets() -> Result<Vec<Docset>, rspc::Error> { // Change return type
    let docsets_connection = docset::open_my_db(&docset::docsets_master_db_path())
        .map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("DB connection failed: {}", e)))?; // Handle error
    // let docsets = docset::search_docsets(&docsets_connection, word);
    let docsets = {
        let con: &Connection = &docsets_connection;
        // let mut stmt = con.prepare("select id, name, alias, feed_url, docset_path, downloaded from docsets").unwrap();
        let mut stmt = con.prepare("select * from docsets")
            .map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("DB prepare failed: {}", e)))?; // Handle error
        let docset_results = stmt
            .query_map(params![], |row| {
                // Using ok() and then unwrap_or_default() or similar might be safer than unwrap() here
                // For now, keeping unwrap but ideally handle potential row.get errors
                Ok(Docset {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    alias: row.get(2).unwrap(),
                    feed_url: row.get(3).unwrap(),
                    docset_path: row.get(4).unwrap(),
                    downloaded: row.get(5).unwrap(),
                })
            })
            .map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("DB query_map failed: {}", e)))?; // Handle error

        let mut _docsets: Vec<Docset> = Vec::new();
        for d in docset_results {
            // Handle potential errors from the query_map iterator
            let docset = d.map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("DB row processing failed: {}", e)))?;
            // println!("{:?}", docset);
            _docsets.push(docset);
        }
        _docsets
    };

    Ok(docsets) // Wrap result in Ok
}

fn search(word: &str) -> Result<SearchResult, rspc::Error> { // Change return type
    if word.is_empty() {
        // Return Ok even for empty result
        return Ok(SearchResult {
            indices: Vec::new(),
        });
    }

    let target_docset_name = "TypeScript";
    // Handle potential errors when opening DB connections and searching
    let docsets_connection = docset::open_my_db(&docset::docsets_master_db_path())
        .map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("Master DB connection failed: {}", e)))?;
    let docsets = docset::search_docsets(&docsets_connection, target_docset_name); // Assuming search_docsets doesn't return Result for now

    let mut result = SearchResult {
        indices: Vec::new(),
    };

    for d in docsets {
        let doc_con = docset::open_my_db(&docset::docset_db_path(target_docset_name))
            .map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("Docset DB connection failed: {}", e)))?;
        // Assuming search_index doesn't return Result for now
        let search_indices = docset::search_index(&doc_con, word, d);

        for index in search_indices {
            result.indices.push(index)
        }
    }

    Ok(result) // Wrap result in Ok
}

#[derive(Type, serde::Deserialize)]
struct ToDownloadDocset {
    name: String,
    feed_url: String,
}

// Change return type to Result<(), rspc::Error>
async fn download_docset(to_download_docset: ToDownloadDocset) -> Result<(), rspc::Error> {
    println!(
        "{}, {}",
        to_download_docset.name, to_download_docset.feed_url
    );

    // Replace panic with Err(rspc::Error::new(...))
    let content = download_feed(&(to_download_docset.feed_url)).await
        .map_err(|why| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("download feed {}: {}", to_download_docset.feed_url, why)))?;

    let url = docset_url_from_feed(&content);

    // let dest_path: String = docset_path(to_download_docset.name);
    // let dest = Path::new(&dest_path);
    let dest_path_str = docsets_base_path();
    let dest = Path::new(&dest_path_str);

    // Replace panic with Err(rspc::Error::new(...))
    download_and_extract(&url, dest).await
        .map_err(|why| rspc::Error::new(rspc::ErrorCode::InternalServerError, format!("download docset {}: {}", url, why)))?;

    Ok(())
}
