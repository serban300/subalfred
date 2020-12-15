// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

pub fn get_metadata_with_id(id: impl Serialize) -> Value {
	rpc("state_getMetadata", Value::Null, id)
}
pub fn get_metadata() -> Value {
	get_metadata_with_id(DEFAULT_ID)
}

pub fn get_runtime_version_with_id(id: impl Serialize) -> Value {
	rpc("state_getRuntimeVersion", Value::Null, id)
}
pub fn get_runtime_version() -> Value {
	get_runtime_version_with_id(DEFAULT_ID)
}

pub fn get_storage_with_id(
	key: impl Serialize,
	at: Option<impl Serialize>,
	id: impl Serialize,
) -> Value {
	rpc(
		"state_getStorage",
		json!([
			key,
			at.map(|at| serde_json::to_value(at).unwrap())
				.unwrap_or(Value::Null)
		]),
		id,
	)
}
pub fn get_storage(key: impl Serialize, at: Option<impl Serialize>) -> Value {
	get_storage_with_id(key, at, DEFAULT_ID)
}
#[cfg(feature = "raw-params")]
pub fn get_storage_with_raw_params_and_id(params: impl Serialize, id: impl Serialize) -> Value {
	rpc("state_getStorage", params, id)
}
#[cfg(feature = "raw-params")]
pub fn get_storage_with_raw_params(params: impl Serialize) -> Value {
	get_storage_with_raw_params_and_id(params, DEFAULT_ID)
}

pub fn subscribe_storage_with_id(keys: impl Serialize, id: impl Serialize) -> Value {
	rpc("state_subscribeStorage", json!([keys]), id)
}
pub fn subscribe_storage(keys: impl Serialize) -> Value {
	subscribe_storage_with_id(keys, DEFAULT_ID)
}
#[cfg(feature = "raw-params")]
pub fn subscribe_storage_with_raw_params_and_id(
	params: impl Serialize,
	id: impl Serialize,
) -> Value {
	rpc("state_subscribeStorage", params, id)
}
#[cfg(feature = "raw-params")]
pub fn subscribe_storage_with_raw_params(params: impl Serialize) -> Value {
	subscribe_storage_with_raw_params_and_id(params, DEFAULT_ID)
}
