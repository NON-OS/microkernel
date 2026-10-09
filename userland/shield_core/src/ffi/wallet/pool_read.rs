//! The pool's history as this wallet last read it, kept in memory so the next read asks only
//! for the blocks since. It is the public chain, the same for every account, so it outlives a
//! lock; it never leaves memory.

use super::Wallet;
use crate::error::NetError;
use crate::net::pool::RPCS;
use crate::net::rpc::progress::Stop;
use crate::net::tor::Tor;
use crate::wallet::{fetch_history_until, History};
use std::sync::Arc;

impl Wallet {
    /// The pool's history now, read on from the one kept, which it then replaces.
    pub(super) fn pool_history(&self, tor: &Tor, stop: Stop<'_>) -> Result<Arc<History>, NetError> {
        let kept = self.read.lock().map_err(|_| NetError::Transport)?.clone();
        let history = Arc::new(fetch_history_until(tor, &RPCS, stop, kept.as_deref())?);
        if let Ok(mut slot) = self.read.lock() {
            *slot = Some(Arc::clone(&history));
        }
        Ok(history)
    }
}
