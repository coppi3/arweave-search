# Arweave Search
Arweave Seach is a simple tool that allows you to easily search through files stored on the Arweave blockchain.  
It heavily relies on Arweave's GraphQL API but makes it super easy for less tech-savy users to find the files with specific file extensions (and much more filters yet to come!)

## What works now
- [x] Scrapper microservice with a yet single route (localhost:3000/fetch_by_filetype). It allows users to fetch for last 100 files of a particular filetype (Not all filetypes are supported)

## What will work in the future
- [] Crawler that will get metadata of found files to make better filtering options
- [] API that will do all the searching itself, probably will use something like OpenSearch 
- [] Cache that will store other people found files to make serach even faster
- [] Aesthetically-pleasing frontend using tauri and unknown for now JS-framework
