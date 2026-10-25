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

//! Each OperationRegion space (ACPI 6.5, 19.6.100's RegionSpace values),
//! how AML reaches it from this capsule, and why. Pure: `region.rs` takes
//! the path from here, and `capsule_acpi_proofs` holds every space to one
//! row. PlatformRtMechanism's value 0x0B is recalled, unverified.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// A SystemMemory window the kernel mapped under its region rule.
    Window,
    /// An access the kernel makes itself, asking its rule each time.
    Kernel,
    /// The EC's own two ports, by its handshake.
    Ec,
    Refused,
}

/// EC space is 256 bytes, reached one byte at a time.
pub const EC_LAST: u64 = 0xFF;

pub fn path(space: u8) -> (Path, &'static str) {
    match space {
        0 => (Path::Window, "memory: a window granted under the kernel's region rule"),
        1 => (Path::Kernel, "ports: each in or out made by the kernel, its rule asked again"),
        2 => (Path::Kernel, "config: never a write to a held function or a bridge's routing"),
        3 => (Path::Ec, "EC: its own ports, one byte at offsets 0 to 255"),
        4 => (Path::Refused, "SMBus: no host controller is driven for AML"),
        5 => (Path::Refused, "CMOS: the kernel's clock owns the index and data pair"),
        6 => (Path::Refused, "PCI BAR target: it would reach a BAR a driver holds"),
        7 => (Path::Refused, "IPMI: no BMC interface is driven"),
        8 => (Path::Refused, "GPIO fields: pins are the GPIO capsule's"),
        9 => (Path::Refused, "serial bus: it would reach I2C or SPI devices a driver holds"),
        0x0A => (Path::Refused, "PCC: NONOS rings no doorbell"),
        0x0B => (Path::Refused, "PRM: platform runtime handlers are not run"),
        0x7F => (Path::Refused, "FFixedHW: no fixed hardware is driven through AML"),
        _ => (Path::Refused, "a space with no handler here"),
    }
}

pub fn ec_ok(address: u64, width: u32) -> bool {
    address <= EC_LAST && width == 1
}
