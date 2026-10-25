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

//! The words a refusal's log line uses.

use super::facts::MemKind;
use super::refusal::{Hardware, Legacy, Refusal};

impl Refusal {
    pub fn reason(&self) -> &'static str {
        match self {
            Refusal::Empty => "empty",
            Refusal::Wraps => "wraps",
            Refusal::Memory(MemKind::Ram) => "allocator RAM",
            Refusal::Memory(_) => "memory type",
            Refusal::Claimed => "claimed by another capsule",
            Refusal::Protected(Hardware::LocalApic) => "local APIC",
            Refusal::Protected(Hardware::IoApic) => "I/O APIC",
            Refusal::Protected(Hardware::Hpet) => "HPET",
            Refusal::Protected(Hardware::Iommu) => "IOMMU",
            Refusal::Protected(Hardware::Ecam) => "PCI configuration window",
            Refusal::Protected(Hardware::Msix) => "MSI-X table or pending bits",
            Refusal::Protected(Hardware::PowerControl) => "FADT power control register",
            Refusal::Legacy(Legacy::Dma) => "DMA controller ports",
            Refusal::Legacy(Legacy::Pic) => "PIC ports",
            Refusal::Legacy(Legacy::Pit) => "PIT ports",
            Refusal::Legacy(Legacy::PciConfig) => "PCI configuration ports",
            Refusal::Legacy(Legacy::Rtc) => "CMOS clock ports",
            Refusal::Legacy(Legacy::Serial) => "kernel serial port",
            Refusal::Legacy(Legacy::Reset) => "system control port A (reset)",
            Refusal::Legacy(Legacy::Console) => "kernel console CRT ports",
            Refusal::Legacy(Legacy::Debug) => "kernel debug ports",
            Refusal::Legacy(Legacy::ResetRegister) => "FADT reset register",
            Refusal::Bridge => "bridge bus number, window or bus master",
        }
    }
}
