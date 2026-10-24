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

//! The kernel's capability names, each at the index of its bit in a
//! `caps_bits` mask (`src/capabilities/types/defs.rs`), and the six that no
//! role may ever hold. Names, not bits, cross the wire, so this list is how
//! a host knows a name: a name not on it is unknown.

/// Every kernel capability, at the index of its bit.
#[rustfmt::skip]
pub const KERNEL_CAPS: [&str; 37] = [
    "CoreExec", "IO", "Network", "IPC", "Memory", "Crypto", "FileSystem", "Hardware",
    "Debug", "Admin", "RegisterService", "GraphicsDisplayQuery", "GraphicsSurfaceCreate",
    "GraphicsSurfaceMap", "GraphicsPresent", "DeviceEnum", "Driver", "Mmio", "Irq", "Dma",
    "Pio", "InputSource", "TimeSet", "SpawnBroker", "SpawnWindow", "ProcessControl",
    "StoreWrite", "EnrolDevRoot", "Keyring", "Entropy", "AppInstall", "AttestRead",
    "ForeignExec", "LocalSign", "StreamImport", "DeviceSecret", "InputConsumer",
];

/// Never in any ceiling: a login must never be able to mint authority or
/// reach the machine's own secrets (nsh-v1-refusals.md).
pub const FORBIDDEN: [&str; 6] =
    ["Admin", "Debug", "DeviceSecret", "EnrolDevRoot", "Keyring", "LocalSign"];

/// The bit a capability name stands for, or `None` for a name the kernel
/// does not have.
pub fn bit(name: &[u8]) -> Option<u32> {
    let at = KERNEL_CAPS.iter().position(|n| n.as_bytes() == name)?;
    u32::try_from(at).ok()
}

/// The mask of the forbidden capabilities.
pub fn forbidden_mask() -> u64 {
    FORBIDDEN.iter().filter_map(|n| bit(n.as_bytes())).fold(0, |m, b| m | 1 << b)
}
