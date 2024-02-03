/* eslint-disable */
// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "app.docsets", input: never, result: Docset[] } | 
        { key: "app.getAppName", input: never, result: string } | 
        { key: "app.search", input: string, result: SearchResult },
    mutations: 
        { key: "app.download_docset", input: ToDownloadDocset, result: null },
    subscriptions: never
};

export type SearchResult = { indices: SearchIndex[] }

export type SearchIndex = { id: number; name: string; doctype: string; htmlpath: string; docset_name: string }

export type ToDownloadDocset = { name: string; feed_url: string }

export type Docset = { id: number; name: string; alias: string; feed_url: string; docset_path: string; downloaded: boolean }
