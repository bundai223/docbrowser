#!/bin/bash

file="docsets.db"
sqlite3 -cmd "create table docsets(id integer, name text, alias text, feed_url text, docset_path text);" $file
sqlite3 -cmd "insert into docsets values(1, 'TypeScript', 'ts', 'https://dummy.com/typescript', 'TypeScript.docset');" $file
