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

//! Opening an account's store: its folder made, the store unlocked or made once.

use std::sync::atomic::Ordering;
use std::sync::Arc;

use nox_shield_core::ffi::Wallet;

use crate::guard::MachineGuard;

use super::folder::dir_of;
use super::held::LOCKS;
use super::open_finish::finish;
use super::Answer;

/// Open the shield of the 0x account `address`: unlock the store it has, or
/// make it from `open` once. The account the store opens must be `address`.
pub(super) fn open_with(address: &str, make: impl FnOnce(&Wallet) -> Result<(), String>) -> Answer {
    let locks = LOCKS.load(Ordering::SeqCst);
    let dir = dir_of(address)?;
    crate::steps::step("making the store's folder");
    std::fs::create_dir_all(&dir).map_err(|_| {
        String::from("This machine will not keep a shield store, not even in memory.")
    })?;
    let live = true;
    let new =
        || Wallet::new(dir.clone(), Arc::new(MachineGuard { live })).map_err(|e| e.to_string());
    crate::steps::step("opening the store");
    let mut wallet = new()?;
    let stored = wallet.stored();
    crate::steps::step(if stored { "unlocking the store" } else { "restoring from the words" });
    match (stored, live) {
        (false, _) => make(&wallet)?,
        (true, false) => wallet.unlock().map_err(|e| e.to_string())?,
        (true, true) => {
            // Kept in memory by a service that has since restarted: its key went
            // with it, so the store is made again from the words.
            if wallet.unlock().is_err() {
                drop(wallet);
                let _ = std::fs::remove_dir_all(&dir);
                std::fs::create_dir_all(&dir)
                    .map_err(|_| String::from("This machine will not keep a shield store."))?;
                wallet = new()?;
                make(&wallet)?;
            }
        }
    }
    finish(wallet, address, dir, live, locks)
}
