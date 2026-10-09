//! Taking back notes whose spend was never settled, against the pool's
//! whole history read over Tor just now. The rule is `wallet::take_back`.

use super::Wallet;
use crate::error::WalletError;
use crate::net::rpc::progress::Stop;
use crate::notes::NoteStatus;
use crate::store::Row;
use crate::wallet::scan_history;
use crate::wallet::take_back::releasable;

impl Wallet {
    /// Read the pool, record what it shows spent, and return to the balance
    /// every note held in flight whose spend the chain never settled. Returns
    /// how many came back.
    pub fn take_back_pending(&self) -> Result<u32, WalletError> {
        self.with(|_| ())?;
        let tor = self.tor()?;
        let history = self.pool_history(&tor, Stop::never())?;
        self.with_mut(|s| {
            let (pending, spent) = {
                let held = s.state().held();
                let scan =
                    scan_history(&history, s.account(), &s.state().pending_deposits(), &held);
                let pending: Vec<[u64; 4]> =
                    held.iter().filter(|n| n.status == NoteStatus::Pending).map(|n| n.cm).collect();
                (pending, scan.spent)
            };
            for cm in &spent {
                s.record(Row::Status { cm: *cm, status: NoteStatus::Spent })?;
            }
            let back = releasable(&pending, &spent);
            for cm in &back {
                s.record(Row::Status { cm: *cm, status: NoteStatus::Unspent })?;
            }
            Ok(u32::try_from(back.len()).unwrap_or(u32::MAX))
        })
        .inspect(|_| self.forget_handoffs())
    }

    /// Every hand-off goes once its notes are back: none of them may be handed to a lander.
    fn forget_handoffs(&self) {
        use crate::store::activity::{Activity, TAKEN_BACK};
        use crate::wallet::kept::{earlier, forget};
        let export = self.export_dir();
        let mut dirs: Vec<std::path::PathBuf> =
            earlier(&export).into_iter().map(|(_, d)| d).collect();
        dirs.push(self.handoff_dir());
        for dir in &dirs {
            if let Ok([tag, _]) = crate::wallet::publish::handoff_nullifiers(dir) {
                let mut a = Activity::new(TAKEN_BACK, crate::wallet::history::clock());
                a.tag = tag;
                let _ = self.with_mut(|s| Ok(crate::wallet::history::once(s, a)?));
            }
            forget(dir);
        }
    }
}
