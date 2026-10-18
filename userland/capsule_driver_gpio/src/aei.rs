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

//! GPIO_EVENT (13) on capsule_acpi: LIST, the controller's `_AEI` pins
//! with their GpioInt flags; SIGNAL, run one pin's event method. Asked as
//! the controller's holder, which the kernel's claim test confirms.

use crate::acpi::call;
use alloc::vec;
use alloc::vec::Vec;

const OP_GPIO_EVENT: u8 = 13;
const LIST: u8 = 0;
const SIGNAL: u8 = 1;

/// The controller's `_AEI` pins with their flags (the GpioInt layout:
/// bit 0 edge, bits 2:1 polarity).
pub fn aei_list(controller: u64) -> Vec<(u16, u16)> {
    let mut req = vec![OP_GPIO_EVENT, LIST];
    req.extend_from_slice(&controller.to_le_bytes());
    let Ok(b) = call(&req) else { return Vec::new() };
    let count = b.get(..2).map_or(0, |c| usize::from(u16::from_le_bytes([c[0], c[1]])));
    let pins = b.get(2..).unwrap_or(&[]).chunks_exact(5).take(count);
    pins.map(|p| (u16::from_le_bytes([p[0], p[1]]), u16::from(p[2]))).collect()
}

pub fn aei_signal(controller: u64, pin: u16) -> i32 {
    let mut req = vec![OP_GPIO_EVENT, SIGNAL];
    req.extend_from_slice(&controller.to_le_bytes());
    req.extend_from_slice(&pin.to_le_bytes());
    call(&req).map_or_else(|e| e, |_| 0)
}
