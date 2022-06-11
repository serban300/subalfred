use thiserror::Error as ThisError;

/// The core libraries' main `Error` type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	#[error(transparent)]
	Cargo(#[from] Cargo),
	#[error(transparent)]
	Generic(#[from] Generic),
	#[error(transparent)]
	Key(#[from] Key),
	#[error(transparent)]
	Node(#[from] Node),
	#[error(transparent)]
	Ss58(#[from] Ss58),
}

/// The core Cargo error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Cargo {
	#[error("[core::cargo] failed to exec `cargo metadata`")]
	ExecMetadataFailed(#[source] cargo_metadata::Error),
	#[error("[core::cargo] failed to open the manifest file")]
	OpenManifestFailed(#[source] cargo_toml::Error),
}

/// The generic error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Generic {
	#[error("{0:?}")]
	AlmostImpossible(&'static str),
	#[error(transparent)]
	Codec(#[from] parity_scale_codec::Error),
	#[error(transparent)]
	Fmt(#[from] std::fmt::Error),
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
}

/// The core key error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Key {
	#[error("[core::key] invalid sub-seed, index out of bound")]
	InvalidSubSeed,
}

/// The core node error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Node {
	#[error("[core::node] failed to get the RPC result")]
	GetRpcResultFailed,
	#[error("[core::node] failed to parse metadata")]
	ParseMetadataFailed(#[source] submetadatan::Error),
	#[error("[core::node] failed to start the node")]
	StartFailed(#[source] std::io::Error),
}

/// The core SS58 error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Ss58 {
	#[error("[core::ss58] invalid address, {address:?}")]
	InvalidAddress { address: String, source: Option<Ss58InvalidAddressSource> },
	#[error("[core::ss58] failed to calculate SS58 address")]
	CalculateSs58AddressFailed(#[source] subcryptor::Error),
}
/// The sub-error type of [`Ss58::InvalidAddress`].
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Ss58InvalidAddressSource {
	#[error("{0:?}")]
	ArrayBytes(array_bytes::Error),
	#[error(transparent)]
	Subcryptor(subcryptor::Error),
}