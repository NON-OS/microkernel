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

//! One bring-up command, logged when it fails, and its R1 status check.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::{Cmd, Host};
use super::super::super::text::Line;
use super::super::ops::say_failed;
use super::super::r1::{errors, state, state_name};

/// Send a command; log and pass on a failure.
pub fn step<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    cmd: &Cmd,
    what: &[u8],
) -> EmmcResult<[u32; 4]> {
    h.send(cmd).inspect_err(|&e| say_failed(h, what, e))
}

/// `r1_ok`, and the card received `cmd` in state `want` (CURRENT_STATE is
/// the state a command found the card in, JEDEC eMMC 5.1 6.13 and SD
/// 4.10.1, as recalled, unverified). A card that answers from another
/// state is not where the sequence put it, and is not followed.
pub fn r1_in<M: Mmio, C: Clock, L: Log>(
    h: &Host<M, C, L>,
    cmd: u8,
    status: u32,
    want: u8,
) -> EmmcResult<()> {
    r1_ok(h, cmd, status)?;
    if state(status) != want {
        h.say(
            Line::new()
                .s(b"CMD")
                .dec(cmd as u64)
                .s(b" found the card in ")
                .s(state_name(status).as_bytes()),
        );
        return Err(EmmcError::BadState(status));
    }
    Ok(())
}

pub fn r1_ok<M: Mmio, C: Clock, L: Log>(h: &Host<M, C, L>, cmd: u8, status: u32) -> EmmcResult<()> {
    if errors(status) != 0 {
        let e = EmmcError::Status { cmd, status };
        h.say(Line::new().s(b"CMD").dec(cmd as u64).s(b" status error ").hex(status as u64));
        return Err(e);
    }
    Ok(())
}
