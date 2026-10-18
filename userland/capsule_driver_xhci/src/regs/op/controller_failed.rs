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
use super::usbsts_read::usbsts_read;
use crate::constants::{USBSTS_HCE, USBSTS_HSE};
/// Whether the controller at `op_base` can no longer be driven: USBSTS
/// reads all ones, as a controller gone from the bus or powered down by
/// firmware reads, or reports a Host System Error or a Host Controller
/// Error, either of which halts it until it is reset (xHCI 1.2 section
/// 5.4.2).
pub fn controller_failed(op_base: u64) -> bool {
    let sts = usbsts_read(op_base);
    sts == u32::MAX || sts & (USBSTS_HSE | USBSTS_HCE) != 0
}
