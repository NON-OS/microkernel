//! After a restore, finding the accounts the phrase was used with, five at a time over their own
//! circuits while the history is fetched. An account with a note or any public use counts, and
//! the search stops after five empty accounts in a row.

use super::Wallet;
use crate::error::WalletError;
use crate::net::rpc::progress::Stop;
use std::sync::atomic::Ordering;

/// Empty accounts in a row that end the search, and so also the width of each parallel step.
const GAP: usize = 5;

impl Wallet {
    /// Look for the accounts of a restored phrase and keep those up to the last one used.
    /// Returns how many accounts the wallet now has, their notes recorded.
    pub fn find_accounts(&self) -> Result<u32, WalletError> {
        let addresses: Vec<[u8; 20]> =
            self.with(|s| s.candidates().iter().map(|c| c.evm().address()).collect())?;
        if addresses.is_empty() {
            return self.with(|s| s.count());
        }
        self.searched.store(0, Ordering::Release);
        let tor = self.tor()?;
        let (history, public) = std::thread::scope(|scope| {
            let history = scope.spawn(|| self.pool_history(&tor, Stop::never()));
            let first = self.check_public(&tor, &addresses, 0);
            (history.join(), first)
        });
        let history = history.map_err(|_| WalletError::Unavailable)??;
        let mut public = public?;
        let mut notes = self.paid_from(&history, 0, public.len())?;
        let found = loop {
            let used = |k: &usize| public.get(*k) == Some(&true) || notes.get(*k) == Some(&true);
            let last = (0..public.len()).rev().find(used);
            let reach = last.map_or(GAP, |k| k.saturating_add(1).saturating_add(GAP));
            if reach <= public.len() || public.len() >= addresses.len() {
                break last.map_or(0, |k| k.saturating_add(1));
            }
            let from = public.len();
            public.extend(self.check_public(&tor, &addresses, from)?);
            notes.extend(self.paid_from(&history, from, public.len())?);
        };
        self.keep_found(&history, found)
    }

    /// How many accounts the search has looked at so far.
    pub fn account_search_progress(&self) -> u32 {
        self.searched.load(Ordering::Acquire)
    }
}
