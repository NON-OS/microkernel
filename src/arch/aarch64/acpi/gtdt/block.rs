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

//! A GT Block: its CntCtlBase and up to eight timer frames of 40 bytes,
//! which must lie inside the block.

use super::super::le::{u32_at, u64_at, u8_at};
use super::super::sdt::TableError;
use super::timer::Timer;
use alloc::vec::Vec;

pub(crate) const BLOCK_MIN: usize = 20;
const FRAME_LEN: usize = 40;
const MAX_FRAMES: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GtFrame {
    pub number: u8,
    pub cnt_base: u64,
    pub cnt_el0_base: u64,
    pub physical: Timer,
    pub virtual_timer: Timer,
    pub common_flags: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GtBlock {
    pub cnt_ctl_base: u64,
    pub frames: Vec<GtFrame>,
}

fn frame(f: &[u8]) -> Option<GtFrame> {
    let timer = |at| Some(Timer { gsiv: u32_at(f, at)?, flags: u32_at(f, at + 4)? });
    Some(GtFrame {
        number: u8_at(f, 0)?,
        cnt_base: u64_at(f, 4)?,
        cnt_el0_base: u64_at(f, 12)?,
        physical: timer(20)?,
        virtual_timer: timer(28)?,
        common_flags: u32_at(f, 36)?,
    })
}

pub(crate) fn block(s: &[u8], at: usize) -> Result<GtBlock, TableError> {
    let bad = TableError::Structure(at);
    let (count, offset) = (u32_at(s, 12).ok_or(bad)?, u32_at(s, 16).ok_or(bad)? as usize);
    if count > MAX_FRAMES || (count > 0 && offset < BLOCK_MIN) {
        return Err(bad);
    }
    let frames = (0..count as usize).map(|i| {
        let f = s.get(offset + i * FRAME_LEN..offset + (i + 1) * FRAME_LEN).ok_or(bad)?;
        frame(f).ok_or(bad)
    });
    Ok(GtBlock {
        cnt_ctl_base: u64_at(s, 4).ok_or(bad)?,
        frames: frames.collect::<Result<_, _>>()?,
    })
}
