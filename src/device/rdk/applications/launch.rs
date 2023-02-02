// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationRequest{
// pub appId: String,
// }

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct LaunchApplicationResponse {}

#[allow(unused_imports)]
use crate::dab::applications::launch::LaunchApplicationRequest;
use crate::dab::applications::launch::LaunchApplicationResponse;
use crate::device::rdk::interface::http_post;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = LaunchApplicationResponse::default();
    // *** Fill in the fields of the struct LaunchApplicationResponse here ***

    let Dab_Request: LaunchApplicationRequest = serde_json::from_str(&_packet).unwrap();

    #[derive(Serialize)]
    struct RdkRequest {
        jsonrpc: String,
        id: i32,
        method: String,
        params: RequestParams,
    }

    #[derive(Serialize)]
    struct RequestParams {
        callsign: String,
    }

    let req_params = RequestParams {
        callsign: Dab_Request.appId,
    };

    let request = RdkRequest {
        jsonrpc: "2.0".into(),
        id: 3,
        method: "org.rdk.RDKShell.launch".into(),
        params: req_params,
    };

    #[derive(Deserialize)]
    struct RdkResponse {
        jsonrpc: String,
        id: i32,
        result: LaunchResult,
    }

    #[derive(Deserialize)]
    struct LaunchResult {
        launchType: String,
        success: bool,
    }

    let json_string = serde_json::to_string(&request).unwrap();
    let response_json = http_post(json_string);

    match response_json {
        Err(err) => {
            println!("Erro: {}", err);

            return Err(err);
        }
        _ => (),
    }

    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}