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

//! spi0 taken for this driver, and the three things asked of it: the host
//! opened (its key back), a transfer, and the device's doorbell.

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, mk_service_lookup};

use super::wire::{call, OP_DOORBELL, OP_OPEN, OP_TRANSFER, TRANSFER_MAX};

const SERVICE: &[u8] = b"driver.spi0";
/// spi0 is spawned first; it may still be bringing its host up.
const LOOKUPS: u32 = 10;
const LOOKUP_MS: u64 = 300;

/// spi0's port and the key its host goes by.
pub fn open() -> Result<(u32, u64), &'static str> {
    let port = (0..LOOKUPS)
        .find_map(|i| {
            if i > 0 {
                let _ = mk_idle_ms(LOOKUP_MS);
            }
            lookup()
        })
        .ok_or("spi_hid: driver.spi0 is not there")?;
    let data = call(port, OP_OPEN, &[]).map_err(|_| "spi_hid: driver.spi0 would not open")?;
    let key = data.get(8..16).ok_or("spi_hid: driver.spi0 gave no key")?;
    Ok((port, u64::from_le_bytes(key.try_into().unwrap_or([0; 8]))))
}

/// `out` clocked out in `mode` at no more than `hz`; as many bytes in.
pub fn transfer(port: u32, mode: u8, hz: u32, out: &[u8]) -> Result<Vec<u8>, i32> {
    if out.is_empty() || out.len() > TRANSFER_MAX {
        return Err(super::wire::E_IO);
    }
    let mut body = Vec::with_capacity(8 + out.len());
    body.extend([mode, 0, 0, 0]);
    body.extend(hz.to_le_bytes());
    body.extend_from_slice(out);
    call(port, OP_TRANSFER, &body)
}

/// Whether the line rang since the last look; `None` when spi0 cannot say.
pub fn doorbell(port: u32, uid: u16, pin: u16) -> Option<bool> {
    let mut body = [0u8; 4];
    body[..2].copy_from_slice(&uid.to_le_bytes());
    body[2..].copy_from_slice(&pin.to_le_bytes());
    let d = call(port, OP_DOORBELL, &body).ok()?;
    let word = |i: usize| d.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    (word(0)? != 0).then_some(word(4)? != 0)
}

fn lookup() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let r = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (r >= 0 && port != 0 && pid != 0).then_some(port)
}
