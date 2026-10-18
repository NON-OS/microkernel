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

//! H, the salt and the diag are the private witness of a family and of its
//! sealed record (plan/41): cleared before the heap takes them back, and
//! MkSealRead's copy cleared once the caller has it.

use core::ptr::write_volatile;

use super::family::Family;
use super::sealed::Sealed;

fn clear(hist: &mut [u32], salt: &mut [u64; 4], diag: &mut [u8; 32]) {
    // SAFETY: each is a valid, aligned element of a slice borrowed mutably.
    hist.iter_mut().for_each(|h| unsafe { write_volatile(h, 0) });
    // SAFETY: as above.
    salt.iter_mut().for_each(|s| unsafe { write_volatile(s, 0) });
    wipe_bytes(diag);
}

pub(super) fn wipe_bytes(b: &mut [u8]) {
    // SAFETY: each is a valid byte of a slice borrowed mutably.
    b.iter_mut().for_each(|x| unsafe { write_volatile(x, 0) });
}

impl Family {
    pub(crate) fn clear_witness(&mut self) {
        clear(&mut self.hist[..], &mut self.salt, &mut self.diag);
    }
}

impl Sealed {
    pub(crate) fn clear_witness(&mut self) {
        clear(&mut self.hist[..], &mut self.salt, &mut self.diag);
    }
}

impl Drop for Family {
    fn drop(&mut self) {
        self.clear_witness();
    }
}

impl Drop for Sealed {
    fn drop(&mut self) {
        self.clear_witness();
    }
}
