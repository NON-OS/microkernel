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

//! The forms a scope row takes (scope.rs), and how one meets a device.

/// DeviceRecord bus kinds (device/bus.rs).
pub(crate) const PCI: u8 = 1;
pub(crate) const ACPI: u8 = 2;

/// What one entry of a row admits.
pub(crate) enum Match {
    /// A PCI function of this vendor whose device id lies in one of the
    /// inclusive ranges, given as lo, hi pairs (kernel_proofs pins the form).
    Ids(u16, &'static [u16]),
    /// A PCI function of this class and subclass; of this vendor and prog-if
    /// when they are Some.
    Class { vendor: Option<u16>, class: u8, sub: u8, progif: Option<u8> },
    /// An ACPI record of this broker class; of this vendor and device when
    /// they are Some.
    Acpi { class: u32, vendor: Option<u16>, device: Option<u16> },
}

/// The fields of a DeviceRecord a row reads.
pub(crate) struct Seen {
    pub bus: u8,
    pub vendor: u16,
    pub device: u16,
    pub class: u32,
    pub pci_class: u8,
    pub pci_sub: u8,
    pub progif: u8,
}

pub(super) fn matches(m: &Match, d: &Seen) -> bool {
    let is = |want: Option<u16>, got: u16| want.is_none_or(|w| w == got);
    match *m {
        Match::Ids(vendor, ranges) => {
            d.bus == PCI
                && d.vendor == vendor
                && ranges.chunks_exact(2).any(|r| r[0] <= d.device && d.device <= r[1])
        }
        Match::Class { vendor, class, sub, progif } => {
            d.bus == PCI
                && is(vendor, d.vendor)
                && (class, sub) == (d.pci_class, d.pci_sub)
                && progif.is_none_or(|p| p == d.progif)
        }
        Match::Acpi { class, vendor, device } => {
            d.bus == ACPI && d.class == class && is(vendor, d.vendor) && is(device, d.device)
        }
    }
}
