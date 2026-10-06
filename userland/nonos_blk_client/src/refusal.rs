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

//! The last request a driver failed, kept for the screen that says why an
//! install stopped. The kernel log line (`device::refused`) is lost on a
//! machine with no serial port; this is not.

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};

/// What a refused request asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Read,
    Write,
    Flush,
}

impl Op {
    pub fn word(self) -> &'static str {
        match self {
            Op::Read => "read",
            Op::Write => "write",
            Op::Flush => "flush",
        }
    }
}

/// One failed request, in 512-byte sectors (none for a flush).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub op: Op,
    pub lba: u64,
    pub sectors: u32,
    pub status: i32,
}

static SET: AtomicBool = AtomicBool::new(false);
static OP: AtomicU32 = AtomicU32::new(0);
static LBA: AtomicU64 = AtomicU64::new(0);
static SECTORS: AtomicU32 = AtomicU32::new(0);
static STATUS: AtomicI32 = AtomicI32::new(0);

pub(crate) fn note(op: Op, lba: u64, sectors: u32, status: i32) {
    OP.store(op as u32, Ordering::Relaxed);
    LBA.store(lba, Ordering::Relaxed);
    SECTORS.store(sectors, Ordering::Relaxed);
    STATUS.store(status, Ordering::Relaxed);
    SET.store(true, Ordering::Release);
}

/// The last request refused since the program started, if any.
pub fn last_refusal() -> Option<Refusal> {
    if !SET.load(Ordering::Acquire) {
        return None;
    }
    Some(Refusal {
        op: match OP.load(Ordering::Relaxed) {
            0 => Op::Read,
            1 => Op::Write,
            _ => Op::Flush,
        },
        lba: LBA.load(Ordering::Relaxed),
        sectors: SECTORS.load(Ordering::Relaxed),
        status: STATUS.load(Ordering::Relaxed),
    })
}
