// use bytes::Bytes;
// // use phf::phf_map;
// // use reqwest::header;
// //
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
// use std::{collections::HashMap, error::Error, fmt::Display, time::Duration};

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component, outbound_http,
};
// mod cors;
mod mimetypes;
mod models;
mod requests;
use crate::mimetypes::mimetypes::{MimeMap, MIMETYPES};
use crate::requests::fetch_by_filetype;
// static ref MIMETYPES MimeMap:MimeMap = MimeMap::MIMETYPES;

// Main router
//
#[http_component]
fn handle_blog_apis_rust(req: Request) -> Result<Response> {
    let mut router = spin_sdk::http::Router::default();
    router.post("/fetch_by_filetype", fetch_by_filetype);
    // router.post("/sponsor", requests::process_sponsor_request);
    // router.post("/subscriptions", newsletter::subscribe);
    // router.get("/search", search::search);
    // router.add("/...", http::Method::OPTIONS, cors::process_preflight);
    router.handle(req)
}
