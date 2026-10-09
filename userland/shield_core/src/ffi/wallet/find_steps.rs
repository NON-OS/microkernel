//! The two steps of the account search: looking at the public accounts of the next five, at once,
//! and keeping the accounts found with their notes recorded from the history already fetched.

use super::sync_each::sync_account;
use super::Wallet;
use crate::error::WalletError;
use crate::evm::{used, Network};
use crate::net::tor::{as_account, Tor};
use crate::wallet::{scan_history, History};
use std::sync::atomic::Ordering;

/// Accounts looked at together.
const STEP: usize = 5;

impl Wallet {
    /// Whether each of the accounts from `from`, five at most, was used on either network.
    pub(super) fn check_public(
        &self,
        tor: &Tor,
        addresses: &[[u8; 20]],
        from: usize,
    ) -> Result<Vec<bool>, WalletError> {
        let batch =
            addresses.get(from..addresses.len().min(from.saturating_add(STEP))).unwrap_or(&[]);
        std::thread::scope(|scope| {
            let runs: Vec<_> = (from..)
                .zip(batch)
                .map(|(k, address)| {
                    scope.spawn(move || {
                        let index = u32::try_from(k).unwrap_or(u32::MAX).saturating_add(1);
                        let seen = as_account(index, || {
                            Ok::<bool, WalletError>(
                                used(tor, Network::Mainnet, address)?
                                    || used(tor, Network::Sepolia, address)?,
                            )
                        });
                        self.searched.fetch_add(1, Ordering::AcqRel);
                        seen
                    })
                })
                .collect();
            runs.into_iter().map(|run| run.join().map_err(|_| WalletError::Unavailable)?).collect()
        })
    }

    /// Whether each candidate from `from` to `to` was paid a note in `history`. Only the accounts
    /// the search reaches are tried, since each costs a key agreement per note in the pool.
    pub(super) fn paid_from(
        &self,
        history: &History,
        from: usize,
        to: usize,
    ) -> Result<Vec<bool>, WalletError> {
        self.with(|s| {
            let reached = s.candidates().get(from..to).unwrap_or(&[]);
            reached
                .iter()
                .map(|c| !scan_history(history, c.account(), &[], &[]).received.is_empty())
                .collect()
        })
    }

    /// Keep the first `found` candidates and record their notes from `history`.
    pub(super) fn keep_found(&self, history: &History, found: usize) -> Result<u32, WalletError> {
        let paths = self.paths().clone();
        self.with_mut(|s| {
            s.keep_candidates(&paths, found)?;
            for index in 1..s.count() {
                sync_account(s, index, history)?;
            }
            Ok(s.count())
        })
    }
}
