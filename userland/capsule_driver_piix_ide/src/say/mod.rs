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

//! The console lines: each disk served, with its model and size; each slot
//! left alone, and why. A disk's serial number is never logged: it names
//! the one machine, and the log can leave it.

mod line;

use crate::disk::Disk;
use line::Line;

const TAG: &[u8] = b"driver.piix_ide0: unit ";

pub fn disk(unit: usize, d: &Disk) {
    let model = &d.id.model;
    let end = model.iter().rposition(|&c| c != b' ').map_or(0, |i| i + 1);
    let mut line = Line::new(TAG);
    line.dec(unit as u64).push(b": ").text(&model[..end]).push(b", ");
    line.dec(d.id.sectors).push(b" sectors (").dec(d.id.sectors / 2048).push(b" MiB)");
    if d.id.lba48 {
        line.push(b", LBA48");
    }
    line.say();
}

pub fn packet(unit: usize) {
    let mut line = Line::new(TAG);
    line.dec(unit as u64).push(b": a packet device (CD/DVD), not served").say();
}

pub fn refused(unit: usize, why: &str) {
    let mut line = Line::new(TAG);
    line.dec(unit as u64).push(b": not served, ").push(why.as_bytes()).say();
}

pub fn no_disk() {
    Line::new(b"driver.piix_ide0: no ATA disk behind the controller, not started").say();
}
