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
use spin::Mutex;

use super::key_slot::{wipe_slot, Link, Slot};

/// The key the current gateway registration produced, in one place and only
/// there. It is never copied out: every frame borrows it under the lock
/// (`with_gateway_key`), and the Gateway record, which is copied into the
/// table and every session, does not carry it.
///
/// It belongs to the connection it was registered on (`Link`). A gateway set
/// on any other connection, or the bound one lost, wipes it where it lies.
/// A fixed slot, not an `Option`: assigning `None` stores a whole `Option`,
/// payload bytes included, so a cleared slot could hold stale bytes.
static SHARED_KEY: Mutex<Slot> = Mutex::new(Slot { key: [0; 32], owner: None });

/// Hold `key` for the connection `owner`, wiping whatever was held.
pub fn set_gateway_shared_key(owner: Link, key: &[u8; 32]) {
    let mut slot = SHARED_KEY.lock();
    wipe_slot(&mut slot);
    slot.key.copy_from_slice(key);
    slot.owner = Some(owner);
}

/// `f` over the key, under the lock, or `None` when no key is held. `f` must
/// not reach this slot again.
pub fn with_gateway_key<R>(f: impl FnOnce(&[u8; 32]) -> R) -> Option<R> {
    let slot = SHARED_KEY.lock();
    slot.owner.is_some().then(|| f(&slot.key))
}

pub fn gateway_key_held() -> bool {
    SHARED_KEY.lock().owner.is_some()
}

/// A gateway is being set on `link`: a key registered on any other
/// connection is wiped.
pub fn keep_gateway_key_for(link: Link) {
    let mut slot = SHARED_KEY.lock();
    if slot.owner != Some(link) {
        wipe_slot(&mut slot);
    }
}

pub fn clear_gateway_shared_key() {
    wipe_slot(&mut SHARED_KEY.lock());
}
