// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The one place a new wallet is written down.
//!
//! Called from generate, import and recover so the three paths cannot drift
//! apart, and so the status line says the same thing about a machine that
//! cannot keep a wallet no matter how the wallet arrived.

use super::keep_plan::{keep_records, kept_status, Kept, Records};
use crate::wallet::state::{State, WORDS_HELD, WORDS_NONE};
use crate::wallet::vault::Unsealed;
use crate::wallet::vault::{forget_vault, forget_words, remember, remember_kind, remember_words};

/// Seal the wallet to this machine and store it, then say which happened.
///
/// A failure here is not a failed wallet. The keys are in the keyring and the
/// session works; what is lost is the next boot, and the words on the backup
/// screen are still the way back. Saying so plainly is better than a silent
/// success the user only discovers is false after a reboot. The records are
/// written in the order `keep_plan` sets, each one checked.
/// `from_key` is a wallet imported from a private key, which has no words.
pub fn keep(state: &mut State, from_key: bool) {
    /* A new wallet in place of another: nothing of the last one stays on
     * screen, as when another account is opened. */
    crate::wallet::state::forget_live(state);
    crate::wallet::shield::open::forget_account(state);
    crate::wallet::accounts::reset_root(state);
    state.words = if from_key { WORDS_NONE } else { WORDS_HELD };
    /* A boot that keeps nothing writes nothing: a cleared vault would still
     * reach the stick there, and the new one never would. */
    let kept = if crate::wallet::vault::persistent() {
        keep_records(&mut Disk { state }, from_key)
    } else {
        Kept::Live
    };
    state.vault_saved = kept == Kept::Kept;
    /* Only a vault that could not be cleared is still on the disk, and it
     * is the last wallet's, which this boot does not hold. */
    state.vault_present = matches!(kept, Kept::LastStays(_));
    state.kept_note = kept_status(kept, from_key).and_then(|s| core::str::from_utf8(s).ok());
    if let Some(status) = kept_status(kept, from_key) {
        state.status = status;
    }
    crate::wallet::shield::open::ensure(state);
}

/// The records of the wallet `state` holds, on this machine's disk.
struct Disk<'a> {
    state: &'a State,
}

impl Records for Disk<'_> {
    fn clear_vault(&mut self) -> Result<(), Unsealed> {
        forget_vault()
    }

    fn kind(&mut self, from_key: bool) -> Result<(), Unsealed> {
        remember_kind(from_key)
    }

    fn words(&mut self) -> Result<(), Unsealed> {
        remember_words(self.state.keyring_port, self.state.owner_pid, self.state.wallet_id)
    }

    fn clear_words(&mut self) -> Result<(), Unsealed> {
        forget_words()
    }

    fn accounts(&mut self) -> Result<(), Unsealed> {
        let s = self.state;
        let count = s.accounts.len().max(1) as u8;
        crate::wallet::vault::remember_accounts(count, s.account_open.min(count - 1))
    }

    fn vault(&mut self) -> Result<(), Unsealed> {
        remember(self.state.keyring_port, self.state.owner_pid, self.state.wallet_id)
    }
}
