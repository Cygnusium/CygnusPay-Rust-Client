use serde::Deserialize;

// base response received from CygnusAPI, which is flattened to
// other structs
#[derive(Deserialize, Debug)]
pub struct BaseResponse {
    pub success: Option<bool>,
    #[serde(rename = "error")]
    pub error_msg: Option<String>
}