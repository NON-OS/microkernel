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

//! The first boot under a new bootloader, once the handover record opened
//! the volume: way 3 wrapped again under this boot's own key, and the record
//! cleared, in the two writes layout.rs `moved` orders.

use super::super::key_header::Sealed;
use super::super::key_wrap::wrap;
use super::super::say::say;
use super::layout::moved;
use super::sector::write_checked;
use crate::crypto::rng::fill_random_bytes_secure;

/// Way 3 wrapped again under `own`, written with the record kept and then
/// without it; None when either write did not hold, and the next boot,
/// finding the record still there, tries again.
pub(super) fn moved_to(own: &[u8; 32], key: &[u8; 32], sector: &[u8; 512]) -> Option<Sealed> {
    let mut nonce = [0u8; 12];
    fill_random_bytes_secure(&mut nonce).ok()?;
    let new = wrap(own, key, nonce)?;
    let (kept, cleared) = moved(sector, &new);
    match write_checked(&kept).and_then(|_| write_checked(&cleared)) {
        Ok(()) => {
            say("[DATA] the handover opened the volume; it now opens under this bootloader");
            Some(new)
        }
        Err(e) => {
            crate::log::warn!("[DATA] the handover opened the volume; moving it: {:?}", e);
            None
        }
    }
}
