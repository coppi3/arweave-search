use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
// use phf::phf_map;
use lazy_static::lazy_static;
// use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, error::Error, time::Duration};

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component, outbound_http,
};
// https://github.com/nestdotland/arweave-rs/blob/main/core/src/lib.rs
/// timeout - is secs
/// A simple Spin HTTP component.
#[http_component]
fn service(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let service_request = match parse_request(&req) {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return Err(e);
        }
    };
    let arweave = Arweave::build("https://arweave.dev".to_string(), 20);

    let files = arweave
        .get_files_with_extension("".to_string(), service_request.filetype)
        .unwrap();
    let out_files = json!({ "matching_files": files });

    let mut res = http::Response::builder()
        .status(200)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Some(Bytes::from(out_files.to_string())))?;
    dbg!(files);

    // res.headers_mut()
    //     .insert(http::header::SERVER, "spin/0.1.0".try_into()?);
    Ok(res)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServiceRequest {
    filetype: String,
}
impl ServiceRequest {
    pub fn from_bytes(b: &Bytes) -> Result<Self> {
        Ok(serde_json::from_slice(&b)?)
    }
}
fn parse_request(req: &Request) -> Result<ServiceRequest> {
    ServiceRequest::from_bytes(req.body().as_ref().unwrap_or(&Bytes::new()))
}

// #[http_component]
pub struct Arweave {
    // client: Client,
    rpc_uri: String,
    timeout: usize,
}

pub type MimeMap = HashMap<&'static str, &'static str>;
lazy_static! {
    static ref MIMETYPES: MimeMap = {
        let mut map = HashMap::new();
        map.insert("txt", "text/plain");
        map.insert(
            "docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        );
        map.insert("doc", "application/msword");
        map.insert("epub", "application/epub+zip");
        map.insert("gif", "image/gif");
        map.insert("png", "image/png");
        map.insert("jpeg", "image/jpeg");
        map.insert("jpg", "image/jpeg");
        map.insert("ppt", "application/vnd.ms-powerpoint");
        map.insert(
            "pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        );
        map.insert("json", "application/json");
        map.insert("pdf", "application/pdf");
        map.insert("mp3", "audio/mpeg");
        map.insert("mpeg", "audio/mpeg");
        map.insert("mp4", "audio/mp4");
        map.insert("rar", "application/vnd.rar");
        map.insert("zip", "application/zip");
        map
    };
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorType,
    message: String,
}
#[derive(Debug)]
pub enum ParseErrorType {
    UnknownType,
    NoMatch,
    OutboundHttpError,
}
impl From<outbound_http::OutboundHttpError> for ParseError {
    fn from(error: outbound_http::OutboundHttpError) -> Self {
        Self {
            kind: ParseErrorType::OutboundHttpError,
            message: error.to_string(),
        }
    }
}
impl ParseError {
    fn from_type(error_type: ParseErrorType) -> Self {
        Self {
            kind: error_type,
            message: "".to_string(),
        }
    }
}
pub trait Parseable {
    fn parse_mime(&self, input: &str) -> Option<&'static str>;
}
impl Parseable for MimeMap {
    fn parse_mime(&self, input: &str) -> Option<&'static str> {
        MIMETYPES.get(input).copied()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Tag {
    name: String,
    value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Node {
    id: String,
    tags: Vec<Tag>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Edge {
    cursor: String,
    node: Node,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    transactions: Transactions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Transactions {
    edges: Vec<Edge>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct GraphQLResponse {
    data: Data,
}
impl Arweave {
    pub fn build(rpc_uri: String, timeout: usize) -> Self {
        Self {
            // client: Client::builder()
            //     // .timeout(Duration::from_secs(timeout as u64))
            //     .build()
            //     .unwrap(),
            rpc_uri,
            timeout,
        }
    }

    // pub fn send_outbound(_req: Request) -> Result<Response> {
    //     let mut res = spin_sdk::outbound_http::send_request(
    //         http::Request::builder()
    //             .method("GET")
    //             .uri("https://random-data-api.fermyon.app/animals/json")
    //             .body(None)?,
    //     )?;
    //     res.headers_mut()
    //         .insert("spin-component", "rust-outbound-http".try_into()?);
    //     println!("{:?}", res);
    //     Ok(res)
    // }
    pub fn get_files_with_extension(
        &self,
        cursor: String,
        filetype: String,
    ) -> Result<Vec<String>, ParseError> {
        let mut out: Vec<String> = Vec::new();
        let mimetype_option = MIMETYPES.parse_mime(&filetype);
        match mimetype_option {
            Some(mt) => {
                dbg!(&mt);
                let query = format!(
                    r#"{{
transactions(
    first: 100
    after: "{}"
    tags: [{{name: "Content-Type", values: "{}"}}]
  ) {{
    edges {{
      cursor
      node {{
        id
        tags {{
          name
          value
        }}
      }}
    }}
  }}
}}"#,
                    cursor, mt
                );
                let body = json!({ "query": query });
                println!("{}", body.to_string());
                let graphql_url = self.rpc_uri.clone() + "/graphql";
                dbg!(&graphql_url);
                // let resp = self
                //     .client
                //     .post(graphql_url)
                //     .body(body.to_string())
                //     .header(reqwest::header::CONTENT_TYPE, "application/json")
                //     .send()
                //     .await?;
                let mut pre_encoded = r#""query": "#.to_string();
                pre_encoded.push_str(&query);
                dbg!(&pre_encoded);
                let encoded_body: String = general_purpose::STANDARD_NO_PAD.encode(pre_encoded);

                let mut resp = spin_sdk::outbound_http::send_request(
                    http::Request::builder()
                        .method("POST")
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .uri(graphql_url)
                        .body(Some(bytes::Bytes::from(body.to_string())))
                        .unwrap(),
                )?;
                resp.headers_mut()
                    .insert("spin-component", "rust-outbound-http".parse().unwrap());
                println!("{:?}", resp);
                let resp_body = match resp.body().as_ref() {
                    Some(bytes) => bytes.slice(..),
                    None => bytes::Bytes::default(),
                };
                dbg!(resp.status());
                // println!("{}", resp.text().await.unwrap());
                // let txs: GraphQLResponse = serde_json::from_str(&resp.text().await?).unwrap();
                let txs: GraphQLResponse = serde_json::from_slice(&resp_body).unwrap();
                for edge in txs.data.transactions.edges {
                    // dbg!(edge);
                    out.push(edge.node.id)
                }
                return Ok(out);
            }
            None => return Err(ParseError::from_type(ParseErrorType::UnknownType)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn get_file() {
    //     // .net doesn't go thru because of cloudflare for whatever reason sometimes so we try
    //     // .dev here
    //     let arweave = Arweave::build("https://arweave.dev".to_string(), 50);
    //     let res = arweave
    //         .get_files_with_extension("".to_string(), "png".to_string())
    //         .await;
    //     dbg!(res);
    // }
    // #[tokio::test]
    // async fn get_info() {
    //     let arweave = Arweave::build("https://arweave.net".to_string(), 10);
    //     let url = arweave.rpc_uri.clone() + "/info";
    //     let resp = arweave.client.get(url).send().await;
    //     println!("{}", resp.unwrap().text().await.unwrap());
    // }
}
