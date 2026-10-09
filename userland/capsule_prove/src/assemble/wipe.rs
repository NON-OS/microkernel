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

//! Clearing what only the device holds once the proof is made: the secret,
//! which bootloader and kernel slot, and where the device sits in the
//! registry. Volatile, because the values are dead to the compiler after this
//! and a plain store to them may be dropped.

use core::ptr;

use nonos_device_attest::{words_of, Path, Witness};

pub fn wipe(w: &mut Witness) {
    let zero = words_of(&[0; 32]);
    // SAFETY: a valid, aligned field of `w`, which is borrowed mutably.
    unsafe { ptr::write_volatile(&mut w.secret, zero) };
    for slot in [&mut w.bootloader, &mut w.kernel] {
        wipe_bytes(&mut slot.digest);
        wipe_path(&mut slot.path);
    }
    wipe_path(&mut w.device);
}

pub fn wipe_bytes(b: &mut [u8]) {
    for x in b.iter_mut() {
        // SAFETY: a valid byte of a slice borrowed mutably.
        unsafe { ptr::write_volatile(x, 0) };
    }
}

fn wipe_path(p: &mut Path) {
    let zero = words_of(&[0; 32]);
    for s in p.siblings.iter_mut() {
        // SAFETY: a valid, aligned element of a vector borrowed mutably.
        unsafe { ptr::write_volatile(s, zero) };
    }
    for r in p.right.iter_mut() {
        // SAFETY: as above.
        unsafe { ptr::write_volatile(r, false) };
    }
}
