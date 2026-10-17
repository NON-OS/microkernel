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

//! What stops a DMA master the broker found through ACPI, not PCI, before
//! the frames it was handed are freed (F-128). A PCI function's bus
//! mastering is turned off in its command register and its function reset;
//! these devices have neither, so each kind needs a stop of its own. A
//! kind with no stop here gets no DMA at all: MkDmaMap refuses it, so a
//! kind admitted later cannot hold frames before its stop is written.
//! Pure: `kernel_proofs` mounts it.

/// The device table's bus kind and class ids this rule reads, restated so
/// it stays pure (`device::bus`, `class::ids`).
pub(crate) const BUS_ACPI: u8 = 2;
pub(crate) const BLOCK: u32 = 0x0010;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stop {
    /// Not a bus master, or not known to be stoppable: no DMA is granted.
    /// I2C hosts move data through a FIFO the CPU or a separate engine
    /// feeds; GPIO controllers, PS/2, the SCI and I2C-HID children move
    /// none.
    NoDma,
    /// An SD/eMMC host (`sdhci_stop`), the only ACPI kind the broker
    /// records as BLOCK: it masters the bus through SDMA and ADMA2.
    SdhciResetAll,
}

/// The stop for an ACPI device of `class`.
pub(crate) const fn acpi_stop(class: u32) -> Stop {
    match class {
        BLOCK => Stop::SdhciResetAll,
        _ => Stop::NoDma,
    }
}

/// Whether MkDmaMap may hand the device frames at all. Every kind found
/// another way keeps its own rule (bus mastering off for PCI).
pub(crate) const fn may_map(bus_kind: u8, class: u32) -> bool {
    bus_kind != BUS_ACPI || !matches!(acpi_stop(class), Stop::NoDma)
}
