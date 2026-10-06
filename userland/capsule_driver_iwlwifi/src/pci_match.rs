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

//! Which PCI function is an Intel Wi-Fi adapter this driver runs, decided from
//! the broker's device record alone so the decision is proven on the host.
//!
//! The legacy interrupt line plays no part in it. UEFI firmware commonly
//! leaves the INTx line register at 0xFF (and some parts report no INTx pin)
//! on machines where the operating system is expected to use MSI or MSI-X;
//! Linux iwlwifi never reads it. Discovery used to skip such a function, so on
//! those laptops the adapter was never found and Wi-Fi was simply absent. The
//! interrupt is chosen at bind time instead (`setup::irq`).

use crate::constants::INTEL_VENDOR_ID;
use crate::firmware::family::family_for_device;

/// PCI class "network controller", subclass "other" (802.11 adapters).
pub const PCI_CLASS_NETWORK: u8 = 0x02;
pub const PCI_SUBCLASS_OTHER: u8 = 0x80;

/// The record fields the decision reads, copied out of a broker
/// `DeviceRecord` by `discover`.
#[derive(Clone, Copy)]
pub struct Candidate {
    pub is_pci: bool,
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
    pub subclass: u8,
    /// BAR0 is a memory BAR with a non-zero size (the register window).
    pub bar0_mmio: bool,
}

/// Whether `c` is an Intel 802.11 function at all, run here or not.
pub fn is_intel_wifi(c: &Candidate) -> bool {
    c.is_pci
        && c.vendor == INTEL_VENDOR_ID
        && c.class == PCI_CLASS_NETWORK
        && c.subclass == PCI_SUBCLASS_OTHER
}

/// Whether `c` is an Intel Wi-Fi function of a family this driver carries
/// firmware for, with its registers in a memory BAR0.
pub fn is_supported_adapter(c: &Candidate) -> bool {
    is_intel_wifi(c) && c.bar0_mmio && family_for_device(c.device).is_some()
}

/// Whether a legacy INTx line is usable: the function has an interrupt pin and
/// firmware routed it (0xFF means "not connected"). The broker refuses an
/// INTx bind otherwise, so the driver does not ask.
pub fn intx_usable(irq_pin: u8, irq_line: u8) -> bool {
    irq_pin != 0 && irq_line != 0xFF
}
