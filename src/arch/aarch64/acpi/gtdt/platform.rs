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

//! The GTDT's platform timers: GT Blocks (type 0, up to eight timer frames
//! of 40 bytes each) and Arm Generic Watchdogs (type 1, 28 bytes). Each
//! structure's 16-bit length must hold its type and stay in the table; a
//! block's frames must lie inside the block.

use super::super::le::{u16_at, u32_at, u64_at, u8_at};
use super::super::sdt::TableError;
use super::block::{block, GtBlock, BLOCK_MIN};
use super::timer::Timer;
use alloc::vec::Vec;

const WATCHDOG_LEN: usize = 28;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Watchdog {
    pub refresh_frame: u64,
    pub control_frame: u64,
    pub timer: Timer,
}

type Platform = (Vec<GtBlock>, Vec<Watchdog>);

pub fn platform_timers(
    t: &[u8],
    fixed: usize,
    count: u32,
    offset: u32,
) -> Result<Platform, TableError> {
    let (mut blocks, mut dogs, mut at) = (Vec::new(), Vec::new(), offset as usize);
    if count > 0 && at < fixed {
        return Err(TableError::Structure(at));
    }
    for _ in 0..count {
        let bad = TableError::Structure(at);
        let len = u16_at(t, at + 1).ok_or(bad)? as usize;
        let s = t.get(at..at.checked_add(len).ok_or(bad)?).ok_or(bad)?;
        match u8_at(s, 0).ok_or(bad)? {
            0 if len >= BLOCK_MIN => blocks.push(block(s, at)?),
            1 if len >= WATCHDOG_LEN => dogs.push(Watchdog {
                refresh_frame: u64_at(s, 4).ok_or(bad)?,
                control_frame: u64_at(s, 12).ok_or(bad)?,
                timer: Timer { gsiv: u32_at(s, 20).ok_or(bad)?, flags: u32_at(s, 24).ok_or(bad)? },
            }),
            0 | 1 => return Err(bad),
            _ if len >= 4 => {}
            _ => return Err(bad),
        }
        at += len;
    }
    Ok((blocks, dogs))
}
