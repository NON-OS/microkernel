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

//! The PCI Command register a driver may write. Pure, and
//! proven on the host (kernel_proofs::pci_command).
//!
//! Only Memory Space (1), Bus Master (2) and Interrupt Disable (10) move.
//! A value made of those bits alone is stored exactly, so a driver can turn
//! any of them off as well as on (A10 gap 2: they were ORed in, and Bus
//! Master could never be cleared). A value carrying other bits must leave
//! those as they are. Bus Master is for DMA: a caller without Dma may not
//! turn it on, or keep it on, through this path. Memory Space is for MMIO
//! in the same way: a caller without Mmio may not turn it on, or keep it on
//! (H-02: a port-only claim, cleared at the claim, stays cleared). At the
//! claim a port-only device has both off (`at_claim`).

pub(crate) const MEMORY_SPACE: u16 = 1 << 1;
pub(crate) const BUS_MASTER: u16 = 1 << 2;
pub(crate) const INTERRUPT_DISABLE: u16 = 1 << 10;
pub(crate) const WRITABLE: u16 = MEMORY_SPACE | BUS_MASTER | INTERRUPT_DISABLE;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Refused {
    /// A bit outside WRITABLE would change.
    Bits,
    /// Bus Master asked for by a caller without Dma.
    BusMaster,
    /// Memory Space asked for by a caller without Mmio.
    MemorySpace,
}

/// The value to store for a write of `new` over `current`.
pub(crate) fn command_value(
    new: u16,
    current: u16,
    may_master: bool,
    may_mmio: bool,
) -> Result<u16, Refused> {
    let desired = if new & !WRITABLE == 0 { (current & !WRITABLE) | new } else { new };
    if (desired ^ current) & !WRITABLE != 0 {
        return Err(Refused::Bits);
    }
    if desired & BUS_MASTER != 0 && !may_master {
        return Err(Refused::BusMaster);
    }
    if desired & MEMORY_SPACE != 0 && !may_mmio {
        return Err(Refused::MemorySpace);
    }
    Ok(desired)
}

/// The Command register a claim leaves: Bus Master off for every claim (the
/// driver turns it on once it holds Dma), and Memory Space off too for a
/// port-only one, whatever firmware left on.
pub(crate) const fn at_claim(current: u16, port_only: bool) -> u16 {
    let off = if port_only { BUS_MASTER | MEMORY_SPACE } else { BUS_MASTER };
    current & !off
}
