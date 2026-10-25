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


//! The kernel's judgement of a capsule's Command write, by its own pure rules
//! mounted by path, for a capsule holding Dma and not Mmio. The register
//! starts as QEMU's firmware leaves a port-only function, I/O and Memory
//! Space decoding (0x0003): a plain write keeps Memory Space on, which such
//! a capsule may not, and was refused on every boot.

use std::cell::Cell;

#[allow(dead_code)]
#[path = "../../../../src/hardware/broker/pci/change.rs"]
mod change;
#[allow(dead_code)]
#[path = "../../../../src/hardware/broker/pci/command_bits.rs"]
mod command_bits;
#[allow(dead_code)]
#[path = "../../../../src/hardware/broker/pci/command_rule.rs"]
mod command_rule;

/// The Command register firmware left.
pub const FIRMWARE: u16 = 0x0003;

thread_local! {
    static REGISTER: Cell<u16> = const { Cell::new(FIRMWARE) };
}

/// The register as the writes so far have left it.
pub fn register() -> u16 {
    REGISTER.with(Cell::get)
}

/// A plain write of `value`: 0, or -1 for a write the kernel refuses.
pub fn plain(value: u16) -> i64 {
    judge(command_bits::plain(value, register()))
}

/// A write of `value` to the bits `mask` names.
pub fn masked(mask: u16, value: u16) -> i64 {
    let now = register();
    judge(change::merge(value, mask, now).and_then(|d| command_bits::exact(d, now)))
}

fn judge(desired: Option<u16>) -> i64 {
    let Some(d) = desired else { return -1 };
    match command_rule::command_value(d, register(), true, false) {
        Ok(v) => {
            REGISTER.with(|r| r.set(v));
            0
        }
        Err(_) => -1,
    }
}
