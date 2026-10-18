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

//! An application command: CMD55 with the card's address, then the ACMD.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::mmc::cmds::rca_arg;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::cmds::APP_CMD;

/// R1 bit 5, APP_CMD: the card takes the next command as an ACMD.
const R1_APP_CMD: u32 = 1 << 5;

/// CMD55 then `acmd`. A card that answers CMD55 without APP_CMD will not
/// read the next command as an ACMD (APP_CMD, card status bit 5, as
/// recalled, unverified).
pub(super) fn app<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    rca: u16,
    acmd: &Cmd,
) -> EmmcResult<[u32; 4]> {
    let r = h.send(&Cmd::new(APP_CMD, rca_arg(rca), Resp::R1))?;
    if r[0] & R1_APP_CMD == 0 {
        return Err(EmmcError::Status { cmd: APP_CMD, status: r[0] });
    }
    h.send(acmd)
}
