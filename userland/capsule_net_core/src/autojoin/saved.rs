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

//! The saved networks. The record is opened with the TPM key once, the first
//! pass that gets this far, and kept until autojoin is done (`forget_saved`),
//! when the list drops and wipes the passphrases: opening it is a TPM round
//! trip, which every pass used to repeat inside the serve loop.

use spin::{Mutex, MutexGuard};

use nonos_wifi_client::{load, SavedError, SavedList};

use super::machine::Step;
use super::tick::say;

/// The saved networks, opened once and kept while autojoin runs.
static SAVED: Mutex<Option<SavedList>> = Mutex::new(None);

/// Drop the saved list, which wipes its passphrases.
pub fn forget_saved() {
    *SAVED.lock() = None;
}

/// The saved list, opened if it is not yet; or the step the pass ends on.
pub(super) fn saved() -> Result<MutexGuard<'static, Option<SavedList>>, Step> {
    let mut saved = SAVED.lock();
    if saved.is_none() {
        match load() {
            Ok(list) => *saved = Some(list),
            Err(SavedError::NoStore) => return Err(Step::NotYet),
            Err(e) => {
                say(b"[NET-CORE] saved Wi-Fi networks not opened: ");
                say(e.text().as_bytes());
                say(b"\n");
                return Err(Step::Done);
            }
        }
    }
    Ok(saved)
}
