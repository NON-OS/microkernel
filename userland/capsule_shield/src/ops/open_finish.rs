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

//! The end of an open: the account chosen, its address said, the store made the open one.

use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use nox_shield_core::ffi::Wallet;
use shield_wire::Values;

use super::account::pick_account;
use super::held::{Open, LOCKS, OPEN};
use super::{refuse, Answer};

/// The store unlocked: choose the account it opens, say its private
/// address, and make it the open store unless the wallet locked meanwhile.
pub(super) fn finish(
    wallet: Arc<Wallet>,
    address: &str,
    dir: String,
    live: bool,
    locks: u64,
) -> Answer {
    crate::steps::step("choosing the account");
    let opened = match pick_account(&wallet, address.trim()) {
        Ok(a) => a,
        Err(why) => {
            let _ = wallet.lock();
            return Err(why);
        }
    };
    crate::steps::step("reading the private address");
    let mut v = Values::new();
    v.put("address", &wallet.receiving_address().map_err(|e| e.to_string())?);
    v.put("kept", if live { "memory" } else { "volume" });
    v.put("took", &crate::steps::end());
    let mut open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?;
    if LOCKS.load(Ordering::SeqCst) != locks {
        drop(open);
        let _ = wallet.lock();
        return refuse("The wallet locked while the shield opened. Nothing is open.");
    }
    *open = Some(Open { address: opened, dir: PathBuf::from(dir), wallet, live });
    Ok(v)
}
