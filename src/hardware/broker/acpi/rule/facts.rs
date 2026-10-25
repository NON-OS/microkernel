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

//! What the grant rule needs to know about the machine, supplied by the
//! kernel's memory map, claim tables and platform discovery (and, in
//! `kernel_proofs`, by a model of them).

use super::refusal::Hardware;

/// How the firmware memory map types a piece of physical address space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemKind {
    /// Usable RAM: the allocator's.
    Ram,
    Reserved,
    AcpiNvs,
    AcpiReclaim,
    /// UEFI runtime services code and data, where firmware puts SMBIOS:
    /// read for a table, never granted as a window.
    EfiRuntime,
    /// No memory map entry covers it, or the map calls it MMIO: device
    /// address space.
    Unmapped,
    /// Any other type (unusable, persistent, loader data); never granted.
    Other,
}

impl MemKind {
    /// Firmware's own memory and device address space.
    pub fn grantable(self) -> bool {
        matches!(
            self,
            MemKind::Reserved | MemKind::AcpiNvs | MemKind::AcpiReclaim | MemKind::Unmapped
        )
    }

    /// Firmware's own memory, where its tables live. Unlike a window,
    /// never device space or memory the map leaves out.
    pub fn holds_tables(self) -> bool {
        matches!(
            self,
            MemKind::Reserved | MemKind::AcpiNvs | MemKind::AcpiReclaim | MemKind::EfiRuntime
        )
    }
}

pub trait Facts {
    /// Calls `each` with the kind of every piece of [base, end).
    fn mem_kinds(&self, base: u64, end: u64, each: &mut dyn FnMut(MemKind));
    /// Whether any of [base, end) is an MMIO window granted to a capsule.
    fn mmio_claimed(&self, base: u64, end: u64) -> bool;
    /// Hardware the kernel programs itself that [base, end) overlaps.
    fn protected(&self, base: u64, end: u64) -> Option<Hardware>;
    /// Whether [base, end) of the port space touches the FADT's reset
    /// register (`power`). PM1 control is granted, its SLP_EN never written.
    fn reset_port(&self, base: u64, end: u64) -> bool;
    /// Whether any of `count` ports from `base` is granted to a capsule.
    fn ports_claimed(&self, base: u16, count: u32) -> bool;
    /// Whether a PCI function (segment, bus, device, function) is claimed.
    fn pci_claimed(&self, bdf: (u16, u8, u8, u8)) -> bool;
}
