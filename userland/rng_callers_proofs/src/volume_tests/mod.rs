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

//! The data volume's keys, uuids, salts and nonces (fs/blockfs_volume/
//! {format_volume,session,passphrase_create}.rs), proved from inside its
//! module, where its `pub(super)` calls are in reach: each is drawn only
//! from the seeded RNG, and with none, nothing is stored, started, written
//! or formatted. The open volume is one global; these tests take turns.

#![warn(clippy::all)]

mod create;
mod format;
mod session;

use std::sync::{Mutex, MutexGuard};

use super::state::VOLUME;

static TURN: Mutex<()> = Mutex::new(());

/// This test's turn, with no volume open and keeping off.
fn turn() -> MutexGuard<'static, ()> {
    let turn = TURN.lock().unwrap_or_else(|p| p.into_inner());
    *VOLUME.write() = None;
    crate::stand_in::block_io::set_keeping(false);
    turn
}

/// The open volume's key, if one is open.
fn open_key() -> Option<[u8; 32]> {
    VOLUME.read().as_ref().map(|v| v.key)
}
