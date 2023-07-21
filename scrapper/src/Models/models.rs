use anyhow::{bail, Result};
extern crate anyhow;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json;

// Model representing payload of a POST / request

#[derive(Deserialize, Debug, PartialEq)]
pub struct FetchByFiletype {
    pub filetype: String,
    pub cursor: Option<String>,
}

impl TryFrom<Option<Bytes>> for FetchByFiletype {
    fn try_from(opt: Option<Bytes>) -> Result<Self, Self::Error> {
        match opt {
            Some(b) => serde_json::from_slice::<Self>(&b).map_err(anyhow::Error::from),
            None => bail!("No body"),
        }
    }
    type Error = anyhow::Error;
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct FetchByFiletypeResponse {
    pub matching_files: Vec<String>,
    pub cursor: String,
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
pub struct Data {
    transactions: Transactions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Transactions {
    edges: Vec<Edge>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphQLResponse {
    pub data: Data,
}

impl TryFrom<Option<Bytes>> for GraphQLResponse {
    fn try_from(opt: Option<Bytes>) -> Result<Self, Self::Error> {
        match opt {
            Some(b) => serde_json::from_slice::<Self>(&b).map_err(anyhow::Error::from),
            None => bail!("No body"),
        }
    }
    type Error = anyhow::Error;
}

impl From<GraphQLResponse> for FetchByFiletypeResponse {
    fn from(resp: GraphQLResponse) -> Self {
        let mut out = Vec::new();
        let mut last_cursor = String::new();
        for edge in resp.data.transactions.edges {
            // dbg!(edge);
            out.push(edge.node.id);
            last_cursor = edge.cursor;
        }
        Self {
            matching_files: out,
            cursor: last_cursor,
        }
    }
}
