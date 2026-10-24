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

/// The key the current gateway registration produced. Held apart from the
/// Gateway record because every frame needs it while only connect sets it, and
/// it must clear on disconnect rather than linger for the next gateway.
///
/// A fixed slot and a flag, not an `Option`: assigning `None` stores a whole
/// `Option`, payload bytes included, and those are whatever the compiler had
/// at hand, so a cleared slot could hold stale stack bytes. Clearing here
/// writes zeros over the key where it lies and nothing else.
static SHARED_KEY: Mutex<Slot> = Mutex::new(Slot { key: [0; 32], set: false });

pub(crate) struct Slot {
    pub(crate) key: [u8; 32],
    pub(crate) set: bool,
}

pub fn set_gateway_shared_key(key: &[u8; 32]) {
    let mut slot = SHARED_KEY.lock();
    slot.key.copy_from_slice(key);
    slot.set = true;
}

pub fn gateway_shared_key() -> Option<[u8; 32]> {
    let slot = SHARED_KEY.lock();
    slot.set.then_some(slot.key)
}

pub fn clear_gateway_shared_key() {
    wipe_slot(&mut SHARED_KEY.lock());
}

pub(crate) fn wipe_slot(slot: &mut Slot) {
    crate::crypto::wipe::wipe(&mut slot.key);
    slot.set = false;
}
