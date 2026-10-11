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

//! SWITCH: send CMD6 for one EXT_CSD byte, and wait for its outcome.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::regs::ERR_DATA_TIMEOUT;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::card::Card;
use super::super::cmds::{switch_arg, SWITCH};
use super::super::r1::errors;
use super::switch_wait::switch_wait;

/// SWITCH EXT_CSD[index] to `value`, and wait until the card has taken it
/// or refused it. Returns the status that ended the wait.
pub fn switch<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &Card,
    index: u8,
    value: u8,
) -> EmmcResult<u32> {
    switch_send(h, card, index, value)?;
    switch_wait(h, card.rca, index, card.switch_ms(index))
}

/// Send CMD6 and let the host wait out its busy phase. A busy phase the host
/// did not see end in time is no failure yet: the status poll that follows
/// decides.
pub fn switch_send<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &Card,
    index: u8,
    value: u8,
) -> EmmcResult<()> {
    let mut c = Cmd::new(SWITCH, switch_arg(index, value), Resp::R1b);
    c.wait_ms = card.switch_ms(index);
    match h.send(&c) {
        Ok(r) if errors(r[0]) != 0 => Err(EmmcError::Status { cmd: SWITCH, status: r[0] }),
        Ok(_) => Ok(()),
        Err(EmmcError::NoCompletion(_)) => Ok(()),
        Err(EmmcError::DataError { err, .. }) if err == ERR_DATA_TIMEOUT => Ok(()),
        Err(e) => Err(e),
    }
}
