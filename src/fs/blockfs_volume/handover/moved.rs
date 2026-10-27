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

//! The machine volume's way-3 open, with the handover (mod.rs). The key this
//! boot's chain gives opens it as before, and a record a prepare left is
//! cleared: the handover lasts one boot. When that key does not open it, a
//! record the handover key opens does, once, and the volume key is wrapped
//! under this boot's key before the record goes (layout.rs `moved`). A write
//! that fails leaves a header the next boot opens; nothing here formats.

use super::super::error::VolumeError;
use super::super::key_header::{Keyed, Sealed};
use super::super::key_header_io::read_key_sector;
use super::super::key_pick::kek;
use super::super::key_wrap::unwrap;
use super::layout::read;
use super::rewrap::moved_to;
use super::sector::{by_record, clear};
use crate::crypto::constant_time::secure_zero;

/// The key the way-3 header `wrapped` holds, and the header the volume is
/// keyed by now. One neither key opens is someone's volume under a chain
/// this boot is not: it is refused, never formatted over.
pub(in super::super) fn opened(wrapped: &Sealed) -> Result<([u8; 32], Keyed), VolumeError> {
    let sector = read_key_sector()?;
    let Some(mut own) = kek() else {
        crate::log::warn!("[DATA] no approved key this boot; the data volume stays closed");
        return Err(VolumeError::Unopenable);
    };
    let opened = match unwrap(&own, wrapped) {
        Some(key) => Some((key, *wrapped, read(&sector).is_some())),
        None => read(&sector).and_then(|r| by_record(&r, &sector)).map(|key| {
            let new = moved_to(&own, &key, &sector);
            (key, new.unwrap_or(*wrapped), false)
        }),
    };
    secure_zero(&mut own);
    let Some((key, now, left)) = opened else {
        crate::log::warn!("[DATA] the volume's key does not unwrap under this chain");
        return Err(VolumeError::Unopenable);
    };
    if left {
        clear(&sector);
    }
    Ok((key, Keyed::Approved(now)))
}
