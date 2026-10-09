// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
//! The order GOP handles are tried in, with no UEFI types so boot_proofs
//! runs this exact file. A laptop with two GPUs has a GOP on each and only
//! one scans out to the panel; taking the first handle that latched could
//! take the GPU with nothing attached, a splash and a kernel framebuffer
//! nobody sees. Firmware installs ConsoleOut on the active console's
//! handle, and Linux's EFI stub (libstub/gop.c, find_gop) takes that GOP
//! first; so does this, keeping the firmware's order otherwise.

use alloc::vec::Vec;

/// Indices into the handle list, the console's handles first.
pub fn console_order(is_console: &[bool]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..is_console.len()).collect();
    order.sort_by_key(|&i| !is_console[i]);
    order
}
