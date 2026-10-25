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

//! The claim table and busy list the model works on (leaving.rs,
//! still_held.rs): the kernel's own table is claim/state.rs, read under a
//! lock that answers shootdowns, which the host cannot mount.

extern crate alloc;

use super::types::Claim;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

pub(crate) static CLAIMS: Mutex<Vec<Claim>> = Mutex::new(Vec::new());
/// Devices a claim or a retire is working on, locked after CLAIMS.
pub(crate) static BUSY: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static EPOCH: AtomicU64 = AtomicU64::new(1);

pub(crate) fn next_epoch() -> u64 {
    EPOCH.fetch_add(1, Ordering::SeqCst)
}
