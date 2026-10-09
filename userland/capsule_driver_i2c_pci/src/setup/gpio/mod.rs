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

//! The touchpad's interrupt line as a doorbell. An i2c-HID device holds its
//! line asserted while an input report waits and releases it once the report
//! is read (HID over I2C v1.0 section 7.4), so reading the pad's input level
//! tells the HID driver when to read, without routing the interrupt.
//!
//! Broxton, Apollo Lake and Gemini Lake declare each GPIO community as its
//! own ACPI controller (INT3452/INT3453, `_UID` 1..4) whose GpioInt pin is
//! the pad index, eight bytes apart from PADBAR (Linux pinctrl-broxton.c,
//! pinctrl-geminilake.c). Sunrise Point and later number pins through
//! per-platform pad groups over several community windows, and AMD has one
//! dword per pin; nonos_pinctrl carries those layouts. Anywhere else the HID
//! driver reads on a timer, which every device supports.
//!
//! Nothing here writes a GPIO register: ownership, routing and the RX

mod bxt;
mod community;
mod doorbell;
mod layout;
mod say;

pub use doorbell::doorbell;
