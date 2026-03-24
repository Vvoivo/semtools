pub mod conversions;
#[allow(clippy::all)]
#[rustfmt::skip] // tonic uses `prettyplease` to format its output
pub mod qdrant;
#[rustfmt::skip] // tonic uses `prettyplease` to format its output
#[path = "grpc.health.v1.rs"]
pub mod grpc_health_v1;
pub mod ops;
pub mod validate;

pub use qdrant::*;

pub const fn api_crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

