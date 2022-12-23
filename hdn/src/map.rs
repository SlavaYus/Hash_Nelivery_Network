use crate::structures::RequestResponse;
use std::{collections::HashMap, sync::MutexGuard};

/// storing
/// ascing in handle_client
///
/// ```
/// pub fn store(
/// storage: &mut MutexGuard<HashMap<String, String>>,
/// key: &str,
/// hash: &str,
/// ) -> RequestResponse {
///     storage.insert(key.to_string(), hash.to_string());
///     RequestResponse::Success {
///         requested_key: String::new(),
///         requested_hash: String::new(),
///     }
/// }
///
///
///
/// ```
///

pub fn store(
    storage: &mut MutexGuard<HashMap<String, String>>,
    key: &str,
    hash: &str,
) -> RequestResponse {
    storage.insert(key.to_string(), hash.to_string());
    RequestResponse::Success {
        requested_key: String::new(),
        requested_hash: String::new(),
    }
}

/// loading
/// ascing in handle_client
///
/// ```
/// pub fn load(storage:  &mut MutexGuard<HashMap<String, String>>, key: &str) -> RequestResponse {
///     match storage.get(key) {
///         None => RequestResponse::KeyNotFound,
///         Some(hash) => RequestResponse::Success {
///             requested_key: key.to_string(),
///             requested_hash: hash.to_string(),
///         }
///     }
/// }
///
///
///
///

pub fn load(storage: &mut MutexGuard<HashMap<String, String>>, key: &str) -> RequestResponse {
    match storage.get(key) {
        None => RequestResponse::KeyNotFound,
        Some(hash) => RequestResponse::Success {
            requested_key: key.to_string(),
            requested_hash: hash.to_string(),
        },
    }
}
