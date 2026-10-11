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

//! The commands of one transfer, sent as its plan says, with their waits.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::{Cmd, Data, Host, Resp};
use super::super::card::Card;
use super::super::cmds::{block_count_arg, SET_BLOCK_COUNT};
use super::super::r1::errors;
use super::plan;
use super::wait_ready::wait_ready;

/// A read of up to 32 KiB finishes within this, garbage collection on the
/// card included.
pub const READ_MS: u64 = 1_000;
/// A write's busy phase ends within this...
pub const WRITE_MS: u64 = 2_000;
/// ...and the card is back in transfer state within this after it. With
/// the recovery, the slowest failing request stays inside the kernel's
/// five-second reply wait.
pub const WRITE_READY_MS: u64 = 1_500;

pub(super) fn run<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &Card,
    buf: DmaBuf,
    arg: u32,
    blocks: u16,
    write: bool,
) -> EmmcResult<()> {
    let p = plan(blocks, write, card.cmd23);
    if p.cmd23 {
        let r = h.send(&Cmd::new(SET_BLOCK_COUNT, block_count_arg(blocks), Resp::R1))?;
        if errors(r[0]) != 0 {
            return Err(EmmcError::Status { cmd: SET_BLOCK_COUNT, status: r[0] });
        }
    }
    let cmd = Cmd {
        index: p.index,
        arg,
        resp: Resp::R1,
        data: Some(Data { read: !write, blocks, multi: p.multi, auto12: p.auto12, buf }),
        abort: false,
        wait_ms: if write { WRITE_MS } else { READ_MS },
    };
    let r = h.send(&cmd)?;
    if errors(r[0]) != 0 {
        return Err(EmmcError::Status { cmd: p.index, status: r[0] });
    }
    if write {
        wait_ready(h, card.rca, WRITE_READY_MS)?;
    }
    Ok(())
}
