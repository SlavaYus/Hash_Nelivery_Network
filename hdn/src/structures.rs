use serde::{Deserialize, Serialize};

/// Struct of incoming name
#[derive(Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

/// Struct of outgoing name
#[derive(Serialize, Deserialize)]
pub struct NameOutput {
    pub student_name: String,
}

///processing incoming requests
/// Example of code:
///
/// ```
/// #[derive(Serialize, Deserialize)]
/// #[serde(tag = "request_type")]
/// #[serde(rename_all = "lowercase")]
/// pub enum ClientRequest {
///     Store { key: String, hash: String },
///     Load { key: String },
/// }
///
/// ```
///
#[derive(Serialize, Deserialize)]
#[serde(tag = "request_type")]
#[serde(rename_all = "lowercase")]
pub enum ClientRequest {
    Store { key: String, hash: String },
    Load { key: String },
}

/// processing outcoming responses with enum
#[derive(Serialize, PartialEq, Debug)]
#[serde(tag = "response_status")]
#[serde(rename_all = "snake_case")]
pub enum RequestResponse {
    Success {
        #[serde(skip_serializing_if = "type_of_request")]
        requested_key: String,
        requested_hash: String,
    },
    KeyNotFound,
}

/// function for serde skip_serializing_if
pub fn type_of_request(v: &str) -> bool {
    v == "load"
}
