//! The pool read directly: this wallet's receiving address, and a sync of the
//! pool's whole history over the wallet's own Tor. The history is fetched without
//! the session lock, since it can take minutes, and only the scan runs under it.
//! A note already held is never recorded again, as a second `Found` would unspend it.

use super::sync_each::{sync_account, sync_watched};
use super::Wallet;
use crate::error::{NetError, WalletError};
use crate::ffi::chain_types::ChainSummary;
use crate::keys::receiving_address_text;
use crate::net::rpc::progress::Stop;
use crate::net::tor::Tor;
use crate::wallet::spend::ripe::waiting;
use std::sync::Arc;

impl Wallet {
    /// The `nox1` address another wallet pays to: spend key, post-quantum key, checksum.
    pub fn receiving_address(&self) -> Result<String, WalletError> {
        self.with(|s| receiving_address_text(&s.account().receiving_address()))
    }

    /// Read the pool's history and record this wallet's notes, deposits and spends. A lock
    /// while it reads stops it at its next request, with `NetError::Stopped`.
    pub fn sync_chain(&self) -> Result<ChainSummary, WalletError> {
        // Taken first, so a lock while Tor starts stops the read too.
        let stop = Stop::watching(&self.stops);
        self.with(|_| ())?;
        let tor = self.tor()?;
        let history = self.pool_history(&tor, stop)?;
        // One history for every account, so the reads do not depend on how many there are.
        self.with_mut(|s| {
            s.leaves_seen = u64::try_from(history.committed.len()).unwrap_or(u64::MAX);
            let mut total = [0u32; 3];
            for index in 0..s.count() {
                let found = sync_account(s, index, &history)?;
                for (sum, n) in total.iter_mut().zip(found) {
                    *sum = sum.saturating_add(n);
                }
            }
            for watched in s.watching.list.iter_mut() {
                sync_watched(watched, &history)?;
            }
            let [received, deposited, spent] = total;
            let unspent: Vec<u64> = s.state().held_here().iter().map(|n| n.leaf_index).collect();
            let (leaves, blocks) = waiting(&history.committed, history.head, &unspent);
            Ok(ChainSummary {
                head: history.head,
                received,
                deposited,
                spent,
                balances: s.balances(),
                wait_leaves: u32::try_from(leaves).unwrap_or(u32::MAX),
                wait_minutes: u32::try_from(blocks.saturating_div(5)).unwrap_or(u32::MAX),
            })
        })
    }
}

impl Wallet {
    /// The wallet's Tor client, bootstrapped once in its own directory, where its guards persist.
    pub(super) fn tor(&self) -> Result<Arc<Tor>, WalletError> {
        let mut slot = self.tor.lock().map_err(|_| NetError::Transport)?;
        let tor = match slot.as_ref() {
            Some(tor) => Arc::clone(tor),
            None => Arc::new(Tor::start(&self.paths().tor)?),
        };
        *slot = Some(Arc::clone(&tor));
        tor.use_account(self.active.load(std::sync::atomic::Ordering::Acquire));
        Ok(tor)
    }
}
