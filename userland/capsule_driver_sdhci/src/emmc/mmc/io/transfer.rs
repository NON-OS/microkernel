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

//! One block transfer: the range check, then the commands, then recovery.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::Host;
use super::super::card::Card;
use super::super::cmds::data_arg;
use super::super::ops::{recover, say_failed};
use super::run::run;

/// The sectors `lba..lba + n` lie on the card, `n` is at least one and the
/// buffer holds them.
pub fn within(card: &Card, lba: u64, n: u32, buf_len: usize) -> bool {
    let Some(end) = lba.checked_add(n as u64) else {
        return false;
    };
    n != 0 && end <= card.sectors && (n as usize).checked_mul(512).is_some_and(|b| b <= buf_len)
}

/// Move `n` sectors at `lba` between the card and `buf` (read into it, or
/// write from it).
pub fn transfer<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &Card,
    buf: DmaBuf,
    lba: u64,
    n: u32,
    write: bool,
) -> EmmcResult<()> {
    if !within(card, lba, n, buf.len) || n > u16::MAX as u32 {
        return Err(EmmcError::OutOfRange);
    }
    let arg = data_arg(lba, card.sector_mode).ok_or(EmmcError::OutOfRange)?;
    let r = run(h, card, buf, arg, n as u16, write);
    if let Err(e) = r {
        say_failed(h, if write { b"write" as &[u8] } else { b"read" }, e);
        recover(h, card.rca);
    }
    r
}
