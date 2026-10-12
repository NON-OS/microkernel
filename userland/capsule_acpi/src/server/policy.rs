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

//! Who may ask each op. The kernel's HELD line admits whole services to
//! the endpoint; this names, op by op, which of them each op answers,
//! checked as HELD checks: by the service a sender owns, its pid asked of
//! the registry at each request so a restart is seen. An op nobody is
//! named for answers no one. Pure: `capsule_acpi_proofs` mounts it.

const POWER: &[u8] = b"power";
const SHELL: &[u8] = b"desktop_shell";
const I2C_PCI: &[u8] = b"driver.i2c_pci0";
const I2C_HID: &[u8] = b"driver.i2c_hid0";
const PS2: &[u8] = b"driver.ps2_kbd0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allowed {
    /// The owners of these services.
    Services(&'static [&'static [u8]]),
    /// Any sender the endpoint admits: DEVICE_POWER, where the kernel's
    /// claim test (MkAcpiFunctionOf, MkAcpiDeviceName) limits a move to
    /// the device's own holder.
    AnyAdmitted,
    Nobody,
}

pub fn allowed(op: u8) -> Allowed {
    match op {
        3 | 4 | 6 | 7 => Allowed::Services(&[POWER, SHELL]),
        5 => Allowed::Services(&[POWER]),
        8 => Allowed::Services(&[SHELL]),
        9 => Allowed::Services(&[I2C_PCI, I2C_HID]),
        10 => Allowed::AnyAdmitted,
        11 => Allowed::Services(&[PS2]),
        _ => Allowed::Nobody,
    }
}

/// Whether `sender` may ask `op`; `owner` is the registry's pid for a
/// service name, None when no one runs it.
pub fn admits(op: u8, sender: u32, owner: impl Fn(&[u8]) -> Option<u32>) -> bool {
    match allowed(op) {
        Allowed::Services(names) => {
            sender != 0 && names.iter().any(|n| owner(n).is_some_and(|p| p != 0 && p == sender))
        }
        Allowed::AnyAdmitted => sender != 0,
        Allowed::Nobody => false,
    }
}
