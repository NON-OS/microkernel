//! The seed at rest, sealed under a file key that the platform keystore wraps.
//! Reading it needs the file and the device.

mod at;
mod destroy;
mod load;
mod nonce;
mod store;
mod words;

pub use at::Vault;
pub use load::Opened;

use nonce::NONCE;
