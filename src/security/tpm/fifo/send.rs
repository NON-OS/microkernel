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

//! Command bytes into the FIFO.

use super::bus::{burst_count, wait_bits, FifoBus};
use super::fail::FifoFail;
use super::regs::{STS_EXPECT, STS_VALID, TIMEOUT_C_MS, TPM_DATA_FIFO, TPM_STS};

/// Write the command, honouring burstCount, and check Expect as the
/// protocol defines it: set after every byte but the last, clear after the
/// last. A part that disagrees has parsed a different length from the
/// header than the one sent, and running that command would run something
/// other than what the caller built.
pub(crate) fn send<B: FifoBus>(bus: &mut B, cmd: &[u8]) -> Result<(), FifoFail> {
    let Some((last, body)) = cmd.split_last() else {
        return Err(FifoFail::CommandTooShort);
    };
    let mut sent = 0;
    while sent < body.len() {
        let burst = burst_count(bus)?;
        let end = body.len().min(sent + burst);
        for byte in &body[sent..end] {
            bus.write8(TPM_DATA_FIFO, *byte);
        }
        sent = end;
        if settled(bus)? & STS_EXPECT == 0 {
            return Err(FifoFail::ExpectDropped);
        }
    }
    burst_count(bus)?;
    bus.write8(TPM_DATA_FIFO, *last);
    if settled(bus)? & STS_EXPECT != 0 {
        return Err(FifoFail::ExpectStillSet);
    }
    Ok(())
}

/// The status after a FIFO access, once stsValid says it is current.
fn settled<B: FifoBus>(bus: &mut B) -> Result<u8, FifoFail> {
    wait_bits(bus, TPM_STS, STS_VALID, STS_VALID, TIMEOUT_C_MS).ok_or(FifoFail::StatusNotValid)
}
