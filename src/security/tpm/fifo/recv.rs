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

//! Response bytes out of the FIFO.

use super::bus::{burst_count, wait_bits, FifoBus};
use super::fail::FifoFail;
use super::regs::{HEADER_LEN, STS_DATA_AVAIL, STS_VALID, TIMEOUT_C_MS, TPM_DATA_FIFO, TPM_STS};

const READABLE: u8 = STS_VALID | STS_DATA_AVAIL;

/// Read the ten-byte header, then exactly the size it names, never more
/// than `out` holds. A part that still has data after that is refused: the
/// header and the bytes disagree and neither can be trusted.
pub(crate) fn receive<B: FifoBus>(bus: &mut B, out: &mut [u8]) -> Result<usize, FifoFail> {
    if out.len() < HEADER_LEN {
        return Err(FifoFail::ResponseSize);
    }
    fill(bus, &mut out[..HEADER_LEN])?;
    let size = u32::from_be_bytes([out[2], out[3], out[4], out[5]]) as usize;
    if size < HEADER_LEN || size > out.len() {
        return Err(FifoFail::ResponseSize);
    }
    fill(bus, &mut out[HEADER_LEN..size])?;
    let sts = wait_bits(bus, TPM_STS, STS_VALID, STS_VALID, TIMEOUT_C_MS)
        .ok_or(FifoFail::StatusNotValid)?;
    if sts & STS_DATA_AVAIL != 0 {
        return Err(FifoFail::ResponseTrailing);
    }
    Ok(size)
}

/// Fill `buf` from the FIFO, a burst at a time, each burst only once the
/// part says data is available.
fn fill<B: FifoBus>(bus: &mut B, buf: &mut [u8]) -> Result<(), FifoFail> {
    let mut got = 0;
    while got < buf.len() {
        wait_bits(bus, TPM_STS, READABLE, READABLE, TIMEOUT_C_MS)
            .ok_or(FifoFail::ResponseStalled)?;
        let burst = burst_count(bus)?;
        let end = buf.len().min(got + burst);
        for slot in &mut buf[got..end] {
            *slot = bus.read8(TPM_DATA_FIFO);
        }
        got = end;
    }
    Ok(())
}
