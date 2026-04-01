use serde::Deserialize;

// Enums

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    ACTIVE,
    INACTIVE,
}

// Structs

// base response received from CygnusAPI, which is flattened to
// other structs
#[derive(Deserialize, Debug)]
pub struct BaseResponse {
    pub success: Option<bool>,
    #[serde(rename = "error")]
    pub error_msg: Option<String>,
}
