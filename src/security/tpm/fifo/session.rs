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

//! One command, start to finish, over any [`FifoBus`].

use super::bus::{wait_bits, FifoBus};
use super::fail::FifoFail;
use super::locality::{relinquish, request};
use super::recv::receive;
use super::regs::{
    HEADER_LEN, STS_COMMAND_READY, STS_DATA_AVAIL, STS_GO, STS_VALID, TIMEOUT_B_MS, TIMEOUT_D_MS,
    TPM_STS,
};
use super::send::send;

/// Take locality 0, run `cmd`, copy the response into `out`, and give the
/// locality back. commandReady is written on every path after the grant:
/// after success it returns the part to idle, after a failure it aborts
/// whatever was half sent or half read, so the next command starts clean.
pub(crate) fn run_command<B: FifoBus>(
    bus: &mut B,
    cmd: &[u8],
    out: &mut [u8],
) -> Result<usize, FifoFail> {
    if cmd.len() < HEADER_LEN {
        return Err(FifoFail::CommandTooShort);
    }
    request(bus)?;
    let result = exchange(bus, cmd, out);
    bus.write8(TPM_STS, STS_COMMAND_READY);
    relinquish(bus);
    result
}

fn exchange<B: FifoBus>(bus: &mut B, cmd: &[u8], out: &mut [u8]) -> Result<usize, FifoFail> {
    make_ready(bus)?;
    send(bus, cmd)?;
    bus.write8(TPM_STS, STS_GO);
    let ready = STS_VALID | STS_DATA_AVAIL;
    wait_bits(bus, TPM_STS, ready, ready, TIMEOUT_D_MS).ok_or(FifoFail::NoResponse)?;
    receive(bus, out)
}

/// Ask for commandReady until it shows, at most twice. From idle one request
/// is enough; a part left mid-command treats the first as an abort and goes
/// idle, and only the second makes it ready.
fn make_ready<B: FifoBus>(bus: &mut B) -> Result<(), FifoFail> {
    for _ in 0..2 {
        if bus.read8(TPM_STS) & STS_COMMAND_READY != 0 {
            return Ok(());
        }
        bus.write8(TPM_STS, STS_COMMAND_READY);
        if wait_bits(bus, TPM_STS, STS_COMMAND_READY, STS_COMMAND_READY, TIMEOUT_B_MS).is_some() {
            return Ok(());
        }
    }
    Err(FifoFail::NotReady)
}
