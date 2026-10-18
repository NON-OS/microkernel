//! The pool's event histories as one read left them.

use crate::net::rpc::RawLog;

/// The pool's three event histories, whole, from the first RPC that serves them so.
#[derive(Clone)]
pub struct History {
    pub head: u64,
    pub committed: Vec<RawLog>,
    pub outputs: Vec<RawLog>,
    pub nullifiers: Vec<RawLog>,
    /// Every `RootCommitted`, for anchoring a spend to the newest root.
    pub roots: Vec<RawLog>,
    /// The newest roots the association registry holds, or none when the pool has no registry.
    pub registered: Option<Vec<[u8; 32]>>,
}
