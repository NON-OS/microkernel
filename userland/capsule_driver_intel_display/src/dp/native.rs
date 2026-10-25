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

//! Native AUX reads and writes of the DPCD, split into transactions of
//! at most 16 bytes, with DEFER retried a bounded number of times (the
//! standard asks a source to retry at least seven times, DisplayPort 1.4a
//! section 2.7.7.1.5; the driver's transfer bounds each try in time).

use super::aux::{header, Aux, AuxError, MAX_DATA, NATIVE_READ, NATIVE_WRITE};
use super::native_once::once;

use super::sideband::refused;

/// Tries per transaction before a DEFER or a timeout counts as failure.
pub const RETRIES: u32 = 8;

/// Read `out.len()` bytes of the DPCD from `addr`.
pub fn read(aux: &mut dyn Aux, addr: u32, out: &mut [u8]) -> Result<(), AuxError> {
    if refused(addr, out.len()) {
        return Err(AuxError::Refused);
    }
    let mut done = 0;
    while done < out.len() {
        let n = (out.len() - done).min(MAX_DATA);
        let req = header(NATIVE_READ, addr + done as u32, n);
        let reply = once(aux, &req)?;
        // An ACK may carry fewer bytes than asked; take what came.
        let got = (reply.1 - 1).min(n);
        if got == 0 {
            return Err(AuxError::Short);
        }
        out[done..done + got].copy_from_slice(&reply.0[1..1 + got]);
        done += got;
    }
    Ok(())
}

/// Write `data` to the DPCD at `addr`.
pub fn write(aux: &mut dyn Aux, addr: u32, data: &[u8]) -> Result<(), AuxError> {
    if refused(addr, data.len()) {
        return Err(AuxError::Refused);
    }
    for (i, chunk) in data.chunks(MAX_DATA).enumerate() {
        let mut req = [0u8; 4 + MAX_DATA];
        req[..4].copy_from_slice(&header(NATIVE_WRITE, addr + (i * MAX_DATA) as u32, chunk.len()));
        req[4..4 + chunk.len()].copy_from_slice(chunk);
        once(aux, &req[..4 + chunk.len()])?;
    }
    Ok(())
}
