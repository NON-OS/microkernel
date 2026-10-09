//! The network side. Every byte leaves the device through a SOCKS5 proxy that
//! the user's platform provides, Orbot or a Nym client, and the policy module
//! refuses an endpoint that would carry the connection in the clear. An RPC
//! from a phone otherwise tells the endpoint which notes belong to which
//! address, which is the leak the proof cannot cover.

pub mod asset;
pub mod asset_v2;
pub mod explorer;
pub mod fee_quote;
pub mod fee_schedule;
pub mod pool;
pub mod pool_errors;
mod pools;
pub mod relay;
pub mod rpc;
pub mod tor;
