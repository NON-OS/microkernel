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

use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::RwLock;

use crate::capabilities::{caps_to_bits, CapabilityToken};
use crate::syscall::contract::allowlist::{for_token, Allowlist};

/// The four PCB fields that make up a process's syscall authority.
pub(crate) struct Cells<'a> {
    pub token: &'a RwLock<Arc<CapabilityToken>>,
    pub list: &'a RwLock<Allowlist>,
    pub bits: &'a AtomicU64,
    pub epoch: &'a AtomicU64,
}

/// What the gate decides a call on: token, list and epoch.
pub struct Snapshot {
    pub token: Arc<CapabilityToken>,
    pub list: Allowlist,
    pub epoch: u64,
}

/*
 * The list is derived (and the token's MAC checked) before the
 * lock, so a gate reader never waits on that work. Bits, list and token are
 * replaced under the token's write lock, so a reader holding the read lock
 * sees one token and the list derived from it.
 */
pub(crate) fn publish(c: &Cells, new: Arc<CapabilityToken>) {
    let list = for_token(&new);
    let mut guard = c.token.write();
    c.bits.store(caps_to_bits(&new.permissions), Ordering::SeqCst);
    *c.list.write() = list;
    *guard = new;
}

/// Nothing permitted until the next publish: the list emptied and the
/// bits cleared, under the token's write lock.
pub(crate) fn withdraw(c: &Cells) {
    let _guard = c.token.write();
    c.bits.store(0, Ordering::SeqCst);
    *c.list.write() = Allowlist::EMPTY;
}

/*
 * Token and list first, under the read lock, then the epoch. A revoke
 * advances the epoch before it publishes, so a reader that saw the new token
 * also sees the new epoch, and a reader that saw the new epoch with the old
 * token is refused as stale. Either way no call is decided on the old token
 * once the advance is visible.
 */
pub(crate) fn snapshot(c: &Cells) -> Snapshot {
    let (token, list) = {
        let token = c.token.read();
        let list = *c.list.read();
        (Arc::clone(&token), list)
    };
    let epoch = c.epoch.load(Ordering::Acquire);
    Snapshot { token, list, epoch }
}
