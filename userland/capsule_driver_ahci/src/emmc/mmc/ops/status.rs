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

//! SEND_STATUS: the card's R1 status.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::cmds::{rca_arg, SEND_STATUS};

/// SEND_STATUS: the card's R1 status.
pub fn status<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>, rca: u16) -> EmmcResult<u32> {
    h.send(&Cmd::new(SEND_STATUS, rca_arg(rca), Resp::R1)).map(|r| r[0])
}
