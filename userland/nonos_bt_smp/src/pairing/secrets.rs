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

//! Pairing's secrets are overwritten when the initiator is dropped, done or
//! not. The key pair clears its own secret.

use super::initiator::Initiator;

fn clear<T: Copy + Default>(v: &mut T) {
    // SAFETY: `v` is a valid, aligned, exclusive reference.
    unsafe { core::ptr::write_volatile(v, T::default()) };
}

impl Drop for Initiator {
    fn drop(&mut self) {
        clear(&mut self.dh_key);
        clear(&mut self.mac_key);
        clear(&mut self.bond.ltk);
        clear(&mut self.passkey);
        clear(&mut self.na);
        clear(&mut self.nb);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
