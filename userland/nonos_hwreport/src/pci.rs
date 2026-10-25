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

//! A PCI function's line: what it is and whether a driver holds it. The
//! line has no field for a bus address, a BAR, a subsystem serial or a
//! device serial number, so none can reach the report.

use crate::sink::Sink;

/// What the broker's record says of one function.
#[derive(Clone, Copy, Debug)]
pub struct Function {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
    pub subclass: u8,
    pub progif: u8,
    pub claimed: bool,
    pub external: bool,
}

/// The line written to `out`; true if it did not fit.
pub fn line(f: &Function, out: &mut Sink) -> bool {
    out.bytes(b"pci ").hex(u64::from(f.vendor), 4).bytes(b":").hex(u64::from(f.device), 4);
    out.bytes(b" class=").hex(u64::from(f.class), 2).bytes(b".");
    out.hex(u64::from(f.subclass), 2).bytes(b".").hex(u64::from(f.progif), 2);
    out.bytes(b" claimed=").bytes(if f.claimed { b"1" } else { b"0" });
    out.bytes(b" external=").bytes(if f.external { b"1" } else { b"0" });
    out.bytes(b"\n").full()
}
