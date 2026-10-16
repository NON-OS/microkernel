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

//! I2C over AUX (DisplayPort 1.4a section 2.7.7.1.6), as the EDID is read
//! from a DisplayPort sink: the offset written to address 0x50, then the
//! bytes read 16 at a time with middle-of-transaction set, and a closing
//! address-only read without it so the sink's I2C bus sees a stop.

use super::aux::{header, i2c_reply, native_reply, Aux, AuxError, Reply};
use super::aux::{I2C_MOT, I2C_READ, I2C_WRITE, MAX_DATA};
use super::native::RETRIES;

/// The EDID's I2C address (VESA E-DDC).
pub const EDID_ADDR: u32 = 0x50;

pub fn once(aux: &mut dyn Aux, req: &[u8]) -> Result<([u8; 20], usize), AuxError> {
    let mut last = AuxError::Timeout;
    for _ in 0..RETRIES {
        let mut r = [0u8; 20];
        match aux.transfer(req, &mut r) {
            Ok(n) if n >= 1 => match (native_reply(r[0]), i2c_reply(r[0])) {
                (Some(Reply::Ack), Some(Reply::Ack)) => return Ok((r, n)),
                (Some(Reply::Defer), _) | (Some(Reply::Ack), Some(Reply::Defer)) => {
                    last = AuxError::Refused
                }
                _ => return Err(AuxError::Refused),
            },
            Ok(_) => last = AuxError::Short,
            Err(e) => last = e,
        }
    }
    Err(last)
}

/// Read `out.len()` bytes from the EDID at byte `offset` (0 or 128 for the
/// first two blocks).
pub fn read_edid(aux: &mut dyn Aux, offset: u8, out: &mut [u8]) -> Result<(), AuxError> {
    let mut w = [0u8; 5];
    w[..4].copy_from_slice(&header(I2C_WRITE | I2C_MOT, EDID_ADDR, 1));
    w[4] = offset;
    once(aux, &w)?;
    let mut done = 0;
    while done < out.len() {
        let n = (out.len() - done).min(MAX_DATA);
        let (r, len) = once(aux, &header(I2C_READ | I2C_MOT, EDID_ADDR, n))?;
        let got = (len - 1).min(n);
        if got == 0 {
            return Err(AuxError::Short);
        }
        out[done..done + got].copy_from_slice(&r[1..1 + got]);
        done += got;
    }
    // Address only, no MOT: the stop that ends the I2C transaction.
    once(aux, &header(I2C_READ, EDID_ADDR, 1)[..3])?;
    Ok(())
}
