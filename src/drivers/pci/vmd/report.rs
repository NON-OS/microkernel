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

//! One VMD's bring-up said on the console: which VMD, where it stopped if it
//! did, what it assigned, and each function found behind it. Each line is
//! built whole and written once, so another core's output cannot split it.

use super::super::types::PciDevice;
use super::domain::Assigned;
use crate::sys::serial::Line;

pub(super) fn announce(dev: &PciDevice) {
    Line::new()
        .str(b"[VMD] ")
        .hex(dev.device_id as u64)
        .str(b" at bus ")
        .dec(dev.address.bus as u64)
        .str(b" dev ")
        .dec(dev.address.device as u64)
        .str(b": RST/VMD is on; bringing up the drives behind it")
        .end();
}

/// Every failed step names itself: the drives behind a VMD that stops here
/// are invisible to every disk driver and the installer.
pub(super) fn hidden(why: &[u8]) {
    Line::new().str(b"[VMD] ").str(why).str(b"; drives behind it stay hidden").end();
}

pub(super) fn report(segment: u16, first: u8, done: &Assigned) {
    let mut line = Line::new();
    line.str(b"[VMD] segment ").hex(segment as u64).str(b" buses ").dec(first as u64);
    line.str(b"-").dec(done.last_bus as u64).str(b" functions ").dec(done.endpoints as u64);
    line.str(b" bars ").dec(done.bars as u64);
    if done.starved != 0 {
        line.str(b" unplaced ").dec(done.starved as u64);
    }
    line.end();
}

/// One function behind a VMD, with its class and ids, so a photo of the log
/// shows whether the NVMe (class 010802) was reached.
pub(super) fn child(dev: &PciDevice) {
    let class = ((dev.class as u64) << 16) | ((dev.subclass as u64) << 8) | dev.progif as u64;
    let ids = ((dev.vendor_id as u64) << 16) | dev.device_id as u64;
    Line::new()
        .str(b"[VMD] found bus ")
        .dec(dev.address.bus as u64)
        .str(b" dev ")
        .dec(dev.address.device as u64)
        .str(b" fn ")
        .dec(dev.address.function as u64)
        .str(b" class ")
        .hex(class)
        .str(b" vendor:device ")
        .hex(ids)
        .end();
}
