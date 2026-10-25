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

//! The GTDT's fixed part: CntControlBase, the four per-processor timers'
//! GSIVs and flags, CntReadBase, where the platform timers are, and, from
//! revision 3, the virtual EL2 timer.

use super::super::le::{u32_at, u64_at};
use super::super::sdt::{revision, table, TableError};
use super::block::GtBlock;
use super::platform::{platform_timers, Watchdog};
use super::timer::Timer;
use alloc::vec::Vec;

/// The fixed part's length before revision 3, and from it.
const FIXED: usize = 96;
const FIXED_V3: usize = 104;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gtdt {
    pub cnt_control_base: u64,
    pub secure_el1: Timer,
    pub nonsecure_el1: Timer,
    pub virtual_el1: Timer,
    pub el2: Timer,
    pub cnt_read_base: u64,
    pub virtual_el2: Option<Timer>,
    pub blocks: Vec<GtBlock>,
    pub watchdogs: Vec<Watchdog>,
}

fn timer(t: &[u8], at: usize) -> Option<Timer> {
    Some(Timer { gsiv: u32_at(t, at)?, flags: u32_at(t, at + 4)? })
}

pub fn gtdt(t: &[u8]) -> Result<Gtdt, TableError> {
    let t = table(t, b"GTDT")?;
    let fixed = if revision(t) >= 3 { FIXED_V3 } else { FIXED };
    if t.len() < fixed {
        return Err(TableError::Short);
    }
    let short = TableError::Short;
    let (count, offset) = (u32_at(t, 88).ok_or(short)?, u32_at(t, 92).ok_or(short)?);
    let (blocks, watchdogs) = platform_timers(t, fixed, count, offset)?;
    Ok(Gtdt {
        cnt_control_base: u64_at(t, 36).ok_or(short)?,
        secure_el1: timer(t, 48).ok_or(short)?,
        nonsecure_el1: timer(t, 56).ok_or(short)?,
        virtual_el1: timer(t, 64).ok_or(short)?,
        el2: timer(t, 72).ok_or(short)?,
        cnt_read_base: u64_at(t, 80).ok_or(short)?,
        virtual_el2: if fixed == FIXED_V3 { timer(t, 96) } else { None },
        blocks,
        watchdogs,
    })
}
