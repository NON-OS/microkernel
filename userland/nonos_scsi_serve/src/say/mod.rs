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

//! The console lines: each unit served, with where it sits, what it is and
//! its size; each unit left alone, and why. A serial number is never
//! logged: it names the one machine, and the log can leave it.

mod line;

use nonos_scsi::{Addr, Disk, Kind};

use line::Line;

fn at(name: &[u8], a: Addr) -> Line {
    let mut line = Line::new(name);
    line.push(b": target ").dec(a.target as u64).push(b" lun ").dec(a.lun as u64);
    line
}

pub fn unit(name: &[u8], unit: usize, d: &Disk) {
    let mut line = Line::new(name);
    line.push(b": unit ").dec(unit as u64).push(b" (target ").dec(d.at.target as u64);
    line.push(b" lun ").dec(d.at.lun as u64).push(b"): ").text(trim(&d.vendor)).push(b" ");
    line.text(trim(&d.product));
    if d.kind == Kind::Cd {
        line.push(b", CD/DVD");
    }
    if d.medium {
        let mib = d.blocks.saturating_mul(d.block_bytes as u64) >> 20;
        line.push(b", ").dec(d.blocks).push(b" x ").dec(d.block_bytes as u64);
        line.push(b" bytes (").dec(mib).push(b" MiB)");
    } else {
        line.push(b", no medium");
    }
    if d.read_only {
        line.push(b", read-only");
    }
    line.say();
}

pub fn other(name: &[u8], a: Addr, device_type: u8) {
    at(name, a).push(b": device type ").dec(device_type as u64).push(b", not served").say();
}

pub fn refused(name: &[u8], a: Addr, why: &str) {
    at(name, a).push(b": not served, ").push(why.as_bytes()).say();
}

pub fn full(name: &[u8], a: Addr) {
    at(name, a).push(b": not served, every unit taken").say();
}

pub fn no_disk(name: &[u8]) {
    Line::new(name).push(b": no disk behind the adapter, not started").say();
}

/// The trailing spaces INQUIRY pads its fields with, cut.
pub fn trim(field: &[u8]) -> &[u8] {
    let end = field.iter().rposition(|&c| c != b' ' && c != 0).map_or(0, |i| i + 1);
    &field[..end]
}
