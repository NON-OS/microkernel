//! The typed failures the core reports, one enum per layer joined at the FFI boundary. No
//! error carries key material, note plaintext or an amount.

mod custody;
mod net;
mod prove;
mod store;
mod wallet;
mod wallet_text;

pub use custody::CustodyError;
pub use net::NetError;
pub use prove::ProveError;
pub use store::StoreError;
pub use wallet::WalletError;
