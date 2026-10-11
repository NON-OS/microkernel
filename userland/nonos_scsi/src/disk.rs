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

//! A logical unit served as a disk: found and sized, read, written and
//! flushed, every command under one retry rule.

use crate::capacity::{self, usable_block, Capacity, RC10_LEN, RC16_LEN};
use crate::cdb;
use crate::errno::{E_INVAL, E_IO, E_NOMEDIUM, E_NXIO, E_ROFS, E_TIMEDOUT};
use crate::hba::{Addr, Dir, Fault, Hba, SENSE_MAX};
use crate::inquiry::{self, STANDARD_LEN, TYPE_CDROM, TYPE_DISK};
use crate::sense::{self, Verdict};
use crate::status::{BUSY, CHECK_CONDITION, CONDITION_MET, GOOD, TASK_SET_FULL};

/// A command is sent at most this many times: a unit attention after a
/// reset is reported once per condition, and a target may hold two.
pub const TRIES: u32 = 4;
/// MODE SENSE (6): the header alone, which carries write protect.
const MODE_HEADER: usize = 4;
const WRITE_PROTECT: u8 = 0x80;
/// VPD 80h: the header and a serial number far longer than the 20 bytes
/// the block wire carries.
const SERIAL_ALLOC: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Disk,
    /// A CD or DVD drive: removable, never written.
    Cd,
}

#[derive(Clone, Copy, Debug)]
pub struct Disk {
    pub at: Addr,
    pub kind: Kind,
    pub blocks: u64,
    pub block_bytes: u32,
    pub removable: bool,
    pub read_only: bool,
    /// Whether a medium is in: always for a fixed disk.
    pub medium: bool,
    pub vendor: [u8; 8],
    pub product: [u8; 16],
    pub serial: [u8; 20],
    pub serial_len: usize,
}

/// Why a logical unit is not served.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Probe {
    /// Nothing there.
    Absent,
    /// A device of another type (a tape, an enclosure...).
    Other(u8),
    Refused(&'static str),
}

/// One command under the retry rule; the bytes the data phase moved.
pub fn command<H: Hba>(
    h: &mut H,
    at: Addr,
    cdb: &[u8],
    dir: Dir,
    len: usize,
) -> Result<usize, i32> {
    for _ in 0..TRIES {
        let o = match h.exec(at, cdb, dir, len) {
            Ok(o) => o,
            Err(Fault::NoTarget) => return Err(E_NXIO),
            Err(Fault::Timeout) => return Err(E_TIMEDOUT),
            Err(Fault::Reset | Fault::Busy) => continue,
            Err(Fault::Failed) => return Err(E_IO),
        };
        match o.status {
            GOOD | CONDITION_MET => return moved(len, o.residual),
            CHECK_CONDITION => {
                match sense::parse(&o.sense[..o.sense_len.min(SENSE_MAX)]).map(sense::verdict) {
                    Some(Verdict::Done) => return moved(len, o.residual),
                    Some(Verdict::Retry) => continue,
                    Some(Verdict::Fail(e)) => return Err(e),
                    None => return Err(E_IO),
                }
            }
            BUSY | TASK_SET_FULL => continue,
            _ => return Err(E_IO),
        }
    }
    Err(E_IO)
}

fn moved(len: usize, residual: u32) -> Result<usize, i32> {
    len.checked_sub(residual as usize).ok_or(E_IO)
}

/// Find out what sits at `at`, and size it.
pub fn probe<H: Hba>(h: &mut H, at: Addr) -> Result<Disk, Probe> {
    let c = cdb::inquiry(None, STANDARD_LEN as u16);
    let n = match command(h, at, c.as_slice(), Dir::In, STANDARD_LEN) {
        Ok(n) => n,
        Err(E_NXIO | E_INVAL) => return Err(Probe::Absent),
        Err(_) => return Err(Probe::Refused("inquiry failed")),
    };
    let i = inquiry::parse(&h.buffer()[..n]).ok_or(Probe::Refused("short inquiry data"))?;
    if i.qualifier != 0 {
        return Err(Probe::Absent);
    }
    let kind = match i.device_type {
        TYPE_DISK => Kind::Disk,
        TYPE_CDROM => Kind::Cd,
        other => return Err(Probe::Other(other)),
    };
    let mut d = Disk {
        at,
        kind,
        blocks: 0,
        block_bytes: 0,
        removable: i.removable || kind == Kind::Cd,
        read_only: kind == Kind::Cd,
        medium: false,
        vendor: i.vendor,
        product: i.product,
        serial: [0; 20],
        serial_len: 0,
    };
    serial(h, &mut d);
    match refresh(h, &mut d) {
        Ok(()) => {}
        Err(E_NOMEDIUM) if d.removable => {}
        Err(E_INVAL) => return Err(Probe::Refused("block size not served")),
        Err(_) => return Err(Probe::Refused("not ready")),
    }
    if kind == Kind::Disk && write_protected(h, at) {
        d.read_only = true;
    }
    Ok(d)
}

/// Whether the unit is ready and how large its medium is now. A removable
/// unit with its medium out answers E_NOMEDIUM and is left with none.
pub fn refresh<H: Hba>(h: &mut H, d: &mut Disk) -> Result<(), i32> {
    d.medium = false;
    command(h, d.at, cdb::test_unit_ready().as_slice(), Dir::None, 0)?;
    let n = command(h, d.at, cdb::read_capacity_10().as_slice(), Dir::In, RC10_LEN)?;
    let mut cap = capacity::rc10(&h.buffer()[..n]).ok_or(E_IO)?;
    if cap == Capacity::TooLarge {
        let c = cdb::read_capacity_16(RC16_LEN as u32);
        let n = command(h, d.at, c.as_slice(), Dir::In, RC16_LEN)?;
        cap = capacity::rc16(&h.buffer()[..n]).ok_or(E_IO)?;
    }
    let Capacity::Known { blocks, block_bytes } = cap else { return Err(E_IO) };
    if !usable_block(block_bytes) {
        return Err(E_INVAL);
    }
    d.blocks = blocks;
    d.block_bytes = block_bytes;
    d.medium = true;
    Ok(())
}

/// The write protect bit of MODE SENSE's header; a unit that does not
/// answer is taken as writable, and a write it refuses says so.
fn write_protected<H: Hba>(h: &mut H, at: Addr) -> bool {
    let c = cdb::mode_sense_6(MODE_HEADER as u8);
    match command(h, at, c.as_slice(), Dir::In, MODE_HEADER) {
        Ok(n) if n >= 3 => h.buffer()[2] & WRITE_PROTECT != 0,
        _ => false,
    }
}

/// The unit serial number, when the unit has the page and it is text.
fn serial<H: Hba>(h: &mut H, d: &mut Disk) {
    let c = cdb::inquiry(Some(cdb::VPD_SERIAL), SERIAL_ALLOC as u16);
    let Ok(n) = command(h, d.at, c.as_slice(), Dir::In, SERIAL_ALLOC) else { return };
    let Some(s) = inquiry::serial(&h.buffer()[..n]) else { return };
    let s = &s[..s.len().min(d.serial.len())];
    if s.iter().all(|c| (0x20..=0x7e).contains(c)) {
        d.serial[..s.len()].copy_from_slice(s);
        d.serial_len = s.len();
    }
}

/// The largest request, in blocks, a buffer of `buffer` bytes carries.
pub const fn max_blocks(d: &Disk, buffer: usize) -> u32 {
    if d.block_bytes == 0 {
        return 0;
    }
    (buffer / d.block_bytes as usize) as u32
}

fn check<H: Hba>(h: &mut H, d: &Disk, lba: u64, count: u32) -> Result<usize, i32> {
    if !d.medium {
        return Err(E_NOMEDIUM);
    }
    let len = count as usize * d.block_bytes as usize;
    if count == 0 || len > h.buffer().len() {
        return Err(E_INVAL);
    }
    match lba.checked_add(count as u64) {
        Some(end) if end <= d.blocks => Ok(len),
        _ => Err(E_NXIO),
    }
}

/// `count` blocks at `lba` into the buffer.
pub fn read<H: Hba>(h: &mut H, d: &Disk, lba: u64, count: u32) -> Result<usize, i32> {
    let len = check(h, d, lba, count)?;
    let c = cdb::rw(false, lba, count).ok_or(E_INVAL)?;
    match command(h, d.at, c.as_slice(), Dir::In, len)? {
        n if n == len => Ok(len),
        _ => Err(E_IO),
    }
}

/// `count` blocks from the buffer to `lba`.
pub fn write<H: Hba>(h: &mut H, d: &Disk, lba: u64, count: u32) -> Result<(), i32> {
    if d.read_only {
        return Err(E_ROFS);
    }
    let len = check(h, d, lba, count)?;
    let c = cdb::rw(true, lba, count).ok_or(E_INVAL)?;
    match command(h, d.at, c.as_slice(), Dir::Out, len)? {
        n if n == len => Ok(()),
        _ => Err(E_IO),
    }
}

/// The unit's write cache to its medium. A unit that has no such command
/// has no cache to flush; a medium never written has nothing in it.
pub fn flush<H: Hba>(h: &mut H, d: &Disk) -> Result<(), i32> {
    if d.read_only || !d.medium {
        return Ok(());
    }
    let c = cdb::synchronize_cache(d.blocks > 1 << 32);
    match command(h, d.at, c.as_slice(), Dir::None, 0) {
        Ok(_) | Err(E_INVAL) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
#[path = "disk_tests.rs"]
mod tests;
