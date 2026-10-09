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

//! The kernel self-attestation context, byte for byte what the bootloader
//! builds, so the tools bind exactly what the gate binds.

use super::constants::BOOT_EPOCH;

/// The kernel self-attestation context: its measurement and the boot epoch.
pub fn kernel_context(kernel_bytes: &[u8]) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(40);
    ctx.extend_from_slice(blake3::hash(kernel_bytes).as_bytes());
    ctx.extend_from_slice(&BOOT_EPOCH.to_be_bytes());
    ctx
}
