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

//! An EDID block over I2C-over-AUX with E-DDC's segment pointer: for block
//! two and on, the segment written to 0x30 with middle-of-transaction set;
//! then the offset written to 0x50, the bytes read 16 at a time with MOT,
//! and the address-only read without MOT that is the stop. The stop goes
//! even after a failure, so the sink's bus is never left mid-transaction.
//! The pointer's place in the sequence is as this driver recalls VESA
//! E-DDC, unverified. No attempt begins after the caller's deadline.

use super::aux::{header, Aux, AuxError, I2C_MOT, I2C_READ, I2C_WRITE, MAX_DATA};
use super::i2c::{once, within, EDID_ADDR};
use crate::engine::access::Expiry;

/// E-DDC's segment pointer.
pub const SEGMENT_ADDR: u32 = 0x30;

/// `out.len()` bytes of segment `segment` from `offset`.
pub fn read_edid(
    aux: &mut dyn Aux,
    segment: u8,
    offset: u8,
    out: &mut [u8],
    until: &dyn Expiry,
) -> Result<(), AuxError> {
    let r = body(aux, segment, offset, out, until);
    let stop = once(aux, &header(I2C_READ, EDID_ADDR, 1)[..3]);
    r.and(stop.map(|_| ()))
}

fn body(
    aux: &mut dyn Aux,
    segment: u8,
    offset: u8,
    out: &mut [u8],
    until: &dyn Expiry,
) -> Result<(), AuxError> {
    if segment > 0 {
        write1(aux, SEGMENT_ADDR, segment, until)?;
    }
    write1(aux, EDID_ADDR, offset, until)?;
    let mut done = 0;
    while done < out.len() {
        let n = (out.len() - done).min(MAX_DATA);
        let (r, len) = within(aux, &header(I2C_READ | I2C_MOT, EDID_ADDR, n), until)?;
        let got = (len - 1).min(n);
        if got == 0 {
            return Err(AuxError::Short);
        }
        out[done..done + got].copy_from_slice(&r[1..1 + got]);
        done += got;
    }
    Ok(())
}

fn write1(aux: &mut dyn Aux, addr: u32, byte: u8, until: &dyn Expiry) -> Result<(), AuxError> {
    let mut w = [0u8; 5];
    w[..4].copy_from_slice(&header(I2C_WRITE | I2C_MOT, addr, 1));
    w[4] = byte;
    within(aux, &w, until).map(|_| ())
}
