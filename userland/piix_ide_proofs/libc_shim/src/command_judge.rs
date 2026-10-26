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

#[path = "../../../../src/hardware/broker/pci/change.rs"]
mod change;
#[path = "../../../../src/hardware/broker/pci/command_bits.rs"]
mod command_bits;
#[path = "../../../../src/hardware/broker/pci/command_rule.rs"]
mod command_rule;

/// What the capsule holds: Dma, and not Mmio.
const DMA: bool = true;
const MMIO: bool = false;

/// The Command register firmware left.
pub const FIRMWARE: u16 = 0x0003;

thread_local! {
    static REGISTER: Cell<u16> = const { Cell::new(FIRMWARE) };
}

/// The register as the writes so far have left it.
pub fn register() -> u16 {
    REGISTER.with(Cell::get)
}

/// A claim answered `rc`: when it holds, the register as the kernel's claim
/// leaves it, Bus Master off and Memory Space too were it port-only.
pub fn claim(rc: i64) -> i64 {
    if rc >= 0 {
        REGISTER.with(|r| r.set(command_rule::at_claim(r.get(), !DMA && !MMIO)));
    }
    rc
}

/// A write of `value` to the Command register at `offset`, of the bits
/// `mask` names (0 for the plain form), as the kernel takes the call's
/// offset argument: 0, or -1 for a write the kernel refuses.
pub fn write(offset: u32, mask: u16, value: u16) -> i64 {
    let now = register();
    let desired = match change::split(offset | ((mask as u32) << change::MASK_SHIFT)) {
        Some((0x04, 0)) => command_bits::plain(value, now),
        Some((0x04, m)) => change::merge(value, m, now).and_then(|d| command_bits::exact(d, now)),
        _ => None,
    };
    let Some(d) = desired else { return -1 };
    match command_rule::command_value(d, now, DMA, MMIO) {
        Ok(v) => {
            REGISTER.with(|r| r.set(v));
            0
        }
        Err(_) => -1,
    }
}
