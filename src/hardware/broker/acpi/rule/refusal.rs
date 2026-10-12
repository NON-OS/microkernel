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

//! Why a region was refused, for the log line the refusal leaves.

use super::facts::MemKind;

/// Kernel-programmed hardware no capsule may map.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hardware {
    LocalApic,
    IoApic,
    Hpet,
    Iommu,
    /// The PCI Express configuration window (MCFG): a memory region over it
    /// would reach a claimed function's configuration around `pci_config`.
    Ecam,
}

/// Legacy devices whose ports the kernel owns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Legacy {
    /// The 8237 DMA controllers and their page registers.
    Dma,
    /// The 8259 interrupt controllers.
    Pic,
    /// The 8254 interval timer.
    Pit,
    /// The configuration address and data ports, and the reset control
    /// register between them.
    PciConfig,
    /// The CMOS index and data pair the kernel's clock reads through; an
    /// index written between the kernel's two accesses reads the wrong byte.
    Rtc,
    /// The first serial port, which carries the kernel's own log.
    Serial,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Empty,
    /// The range wraps the address space, or runs past the 64 KiB of ports.
    Wraps,
    /// Memory of a kind the rule never grants (the allocator's RAM above all).
    Memory(MemKind),
    /// Another capsule holds some of it.
    Claimed,
    Protected(Hardware),
    Legacy(Legacy),
}
