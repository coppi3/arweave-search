// extern crate spin_sdk;
use crate::mimetypes::{Parseable, MIMETYPES};
use crate::models::models::{FetchByFiletype, FetchByFiletypeResponse, GraphQLResponse};
use anyhow::anyhow;
use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use spin_sdk::{
    http::{Params, Request, Response},
    http_component, outbound_http,
};
// #[http_component]
pub fn fetch_by_filetype(req: Request, _params: Params) -> Result<Response> {
    println!("[FETCH_BY_FILETYPE]: Endpoint invoked");
    let Ok(model) = FetchByFiletype::try_from(req.body().clone()) else {
                    return Ok(http::Response::builder()
                      .status(http::StatusCode::BAD_REQUEST)
                      .body(None)?);
        };
    // let body = json!({"filetype": model.filetype});
    let Ok(req) = build_arweave_request(model) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .body(None)?);
    };

    match spin_sdk::outbound_http::send_request(req) {
        Ok(r) => {
            println!("[FETCH_BY_FILETYPE]: Will respond with search results");
            let Ok(new_resp )= handle_graphql_resp(r) else{
            return Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(None)?)

            };
            Ok(new_resp)
        }
        Err(e) => {
            println!(
                    "[Error from FETCH_BY_FILETYPE]: Did not receive successful response from Arweave GraphQL endpoint({})",
                    e
                );
            Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(None)?)
        }
    }
}

fn handle_graphql_resp(resp: Response) -> Result<Response> {
    let Ok(parsed) = GraphQLResponse::try_from(resp.body().clone()) else {
                    return Ok(http::Response::builder()
                      .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                      .body(None)?);};
    let new_resp = FetchByFiletypeResponse::from(parsed);
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Some(Bytes::from(
            serde_json::to_value(new_resp)?.to_string(),
        )))?)
}

fn build_arweave_request(filetype: FetchByFiletype) -> Result<Request> {
    let arweave_rpc = "https://arweave.dev/graphql";

    let mimetype_option = MIMETYPES.parse_mime(&filetype.filetype);
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
                filetype.cursor.unwrap_or(String::from("")),
                mt
            );

            let body = json!({ "query": query });
            println!("{}", body.to_string());
            return Ok(http::Request::builder()
                .method(http::Method::POST)
                .header(http::header::ACCEPT, "application/json")
                .header(http::header::CONTENT_TYPE, "application/json")
                // .header("api-key", search_api_key)
                .uri(arweave_rpc)
                .body(Some(Bytes::from(body.to_string())))?);

            // match spin_sdk::outbound_http::send_request(req) {
            //     Ok(resp) => return Ok(resp),
            //     Err(e) => {
            //         println!("{e}");
            //         return Ok(http::Response::builder()
            //             .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            //             .body(None)?);
            //     }
            // }
        }
        None => {
            return Err(anyhow!("Couldn't find specified type in the lookup table"));
            // return Ok(http::Response::builder()
            //     .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            //     .body(None)?)
        }
    }
}
