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

//! Encoding a platform device record, for the ACPI capsule, which mounts
//! this file, `platform_wire.rs` and `platform_decode.rs` by path so the
//! two sides cannot drift. Pure: `kernel_proofs` proves the round trip.

use super::platform_decode::decode;
use super::platform_wire::Platform;
use alloc::vec;
use alloc::vec::Vec;

fn bytes(p: &Platform) -> Option<Vec<u8>> {
    let n = u8::try_from(p.windows.len()).ok()?;
    let len = u8::try_from(p.path.len()).ok()?;
    let mut b = vec![1, p.class, n, p.irq_flags, p.flags, 0, 0, 0];
    b.extend_from_slice(&p.hid);
    b.extend_from_slice(&p.uid.to_le_bytes());
    b.extend_from_slice(&p.gsiv.to_le_bytes());
    b.extend_from_slice(&[0; 4]);
    for (base, size) in &p.windows {
        b.extend_from_slice(&base.to_le_bytes());
        b.extend_from_slice(&size.to_le_bytes());
    }
    b.push(len);
    b.extend_from_slice(&p.path);
    Some(b)
}

/// The record, or None for a device the kernel would refuse as malformed:
/// what is sent is exactly what decodes back to `p`, and nothing else.
pub fn encode(p: &Platform) -> Option<Vec<u8>> {
    bytes(p).filter(|b| decode(b).as_ref() == Some(p))
}
