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

//! The lines a bring-up logs: a step that failed and why, and each unit it
//! serves or does not.

use crate::ufs::disk::Unit;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::text::Line;

/// "<step> failed: <what the host or device said>".
pub fn failed<H: Hw>(hw: &H, step: &[u8], e: UfsError) -> UfsError {
    let mut l = Line::new();
    l.s(step).s(b" failed: ");
    describe(&mut l, e);
    hw.line(l.as_bytes());
    e
}

pub fn unit<H: Hw>(hw: &H, u: &Unit) {
    let mut l = Line::new();
    l.s(b"LU ").dec(u.lun as u64).s(b": ").dec(u.blocks).s(b" x ").dec(u.block_len as u64);
    l.s(b" bytes, ").ascii(&u.name);
    hw.line(l.as_bytes());
}

pub fn unit_skipped<H: Hw>(hw: &H, lun: u8, e: UfsError) {
    let mut l = Line::new();
    l.s(b"LU ").dec(lun as u64).s(b" not served: ");
    describe(&mut l, e);
    hw.line(l.as_bytes());
}

fn describe(l: &mut Line, e: UfsError) {
    match e {
        UfsError::Timeout => l.s(b"timed out"),
        UfsError::Host => l.s(b"host not ready"),
        UfsError::Uic(r) => l.s(b"UIC result ").hex(r as u64),
        UfsError::NoDevice => l.s(b"no device"),
        UfsError::Ocs(v) => l.s(b"OCS ").hex(v as u64),
        UfsError::Upiu(t) => l.s(b"device answered UPIU ").hex(t as u64),
        UfsError::Query(c) => l.s(b"query response ").hex(c as u64),
        UfsError::Sense { key, asc, ascq } => {
            l.s(b"sense key ").hex(key as u64).s(b" asc ").hex(asc as u64);
            l.s(b" ascq ").hex(ascq as u64)
        }
        UfsError::Status(s) => l.s(b"SCSI status ").hex(s as u64),
        UfsError::Dma32 => l.s(b"DMA above 4 GiB and no 64-bit addressing"),
        UfsError::BlockSize(n) => l.s(b"block size ").dec(n as u64),
        UfsError::Capacity(n) => l.s(b"capacity of ").dec(n).s(b" blocks"),
    };
}
