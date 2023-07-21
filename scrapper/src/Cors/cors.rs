// use crate::constants::{CFG_NAME_CORS_METHODS, CFG_NAME_CORS_ORIGIN};
use anyhow::Result;
use http::response::Builder;
use spin_sdk::{
    config,
    http::{Params, Request, Response},
};
pub fn builder_with_cors(origin: String, methods: String, status: http::StatusCode) -> Builder {
    http::Response::builder()
        .status(status)
        .header("Access-Control-Allow-Origin", origin)
        .header("Access-Control-Allow-Methods", methods)
        .header("Access-Control-Allow-Headers", "*")
}

pub fn process_preflight(_req: Request, _params: Params) -> Result<Response> {
    // let Ok(origin) = config::get(CFG_NAME_CORS_ORIGIN) else {
    //     println!("[ERROR]: Could not find CORS origin");
    //     return Ok(http::Response::builder()
    //         .status(http::StatusCode::INTERNAL_SERVER_ERROR)
    //         .body(None)?);
    // };
    //
    // let Ok(methods) = config::get(CFG_NAME_CORS_METHODS) else {
    //     println!("[ERROR]: Could not find CORS methods");
    //     return Ok(http::Response::builder()
    //         .status(http::StatusCode::INTERNAL_SERVER_ERROR)
    //         .body(None)?);
    // };
    let origin = String::from("*");
    let methods = String::from("*");

    Ok(builder_with_cors(origin, methods, http::StatusCode::OK).body(None)?)
}
