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

//! The two writes of the first boot under a new bootloader, once the
//! handover record (place.rs) opened the volume. Pure.

use super::super::key_header::{Keyed, Sealed};
use super::super::key_seal::encode_key_header;
use super::place::put;

/// Way 3's bytes: magic, way, the bound zeros, its nonce, key and tag.
const WAY3_END: usize = 116;

/// What the first boot under a new bootloader writes, in order, once the
/// record opened the volume: way 3 wrapped again under this boot's key with
/// the record kept, then the same without it. Each is read back before the
/// next is written, so the disk holds a header some boot opens throughout:
/// the old wrap and the record, the new wrap and the record, the new wrap.
pub(super) fn moved(s: &[u8; 512], new: &Sealed) -> ([u8; 512], [u8; 512]) {
    let mut kept = *s;
    kept[..WAY3_END].copy_from_slice(&encode_key_header(&Keyed::Approved(*new))[..WAY3_END]);
    (kept, put(&kept, None))
}
