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

//! An EDID block read over DDC through GMBUS. Blocks two and on sit behind
//! the E-DDC segment pointer at 0x30: its byte is written, with no stop,
//! before the read from 0x50 at the block's index, and the pointer is back
//! at 0 after every stop, so the first two blocks need none (VESA E-DDC,
//! as this driver recalls it; unverified against the standard). A NAK
//! clears the controller and is reported; every wait draws on the
//! caller's deadline.

use super::access::{Clock, Expiry, Mmio};
use super::error::Error;
use super::gmbus_regs::*;
use super::gmbus_xfer::{end::end, put::put, take::take};

pub const EDID_ADDR: u32 = 0x50;
pub const SEGMENT_ADDR: u32 = 0x30;
pub const BLOCK: usize = 128;

/// Block `n` of the EDID on DDI `ddi`'s DDC pins: 128 bytes from index
/// `n % 2 * 128` of segment `n / 2`.
pub fn read_block(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    ddi: u32,
    n: u8,
    out: &mut [u8; BLOCK],
    until: &dyn Expiry,
) -> Result<(), Error> {
    let p = pin(ddi).ok_or(Error::Argument)?;
    m.write(GMBUS0, p);
    let r = point(m, n / 2, until).and_then(|()| {
        m.write(GMBUS1, read_cmd(EDID_ADDR, n % 2 * 128, BLOCK));
        take(m, until, out)
    });
    end(m, clock, r.is_err());
    r
}

/// The segment pointer at `segment`, left in the wait phase so the read
/// follows with a repeated start; a sink without E-DDC NAKs it.
fn point(m: &mut dyn Mmio, segment: u8, until: &dyn Expiry) -> Result<(), Error> {
    match segment {
        0 => Ok(()),
        s => put(m, until, SEGMENT_ADDR, &[s]),
    }
}
