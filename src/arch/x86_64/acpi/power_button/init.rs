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

//! Whether the button can be polled at all, decided once from the FADT.

use core::sync::atomic::Ordering;

use super::super::hw::gas::gas_read;
use super::super::hw::port_bus::PortBus;
use super::super::hw::power_button::{acpi_mode, classify, clear_value, Button};
use super::super::hw::power_button_arm::io_enable_port;
use super::super::parser;
use super::{say, EN_A, EN_B, STS_A, STS_B};
use crate::arch::x86_64::port::outw;

/// Decide once, after the ACPI tables are parsed, whether the button can be
/// polled, and say in one line what the power button will do.
pub fn init() {
    let Some(fadt) = parser::with_data(|d| d.fadt).flatten() else {
        say(b"[ACPI] power button: no FADT, the button stays with the firmware");
        return;
    };
    match classify(&fadt) {
        Button::Fixed { sts_a, sts_b } => {
            let cnt = gas_read(&mut PortBus, &fadt.pm1a_cnt).unwrap_or(0) as u16;
            if !acpi_mode(cnt) {
                say(b"[ACPI] power button: the firmware left ACPI mode off (SCI_EN clear), so the button stays with the firmware");
                return;
            }
            // A press from before boot is not a request to shut down now.
            // SAFETY: the port is the FADT's PM1 status register, which is
            // write-one-to-clear; only the power button bit is written.
            unsafe {
                outw(sts_a, clear_value());
                if sts_b != 0 {
                    outw(sts_b, clear_value());
                }
            }
            EN_A.store(io_enable_port(&fadt.pm1a_evt).unwrap_or(0), Ordering::Relaxed);
            EN_B.store(if sts_b != 0 { io_enable_port(&fadt.pm1b_evt).unwrap_or(0) } else { 0 }, Ordering::Relaxed);
            STS_B.store(sts_b, Ordering::Relaxed);
            STS_A.store(sts_a, Ordering::Release);
            say(b"[ACPI] power button: fixed feature, polled; a press goes to the desktop as the Power key");
        }
        Button::ControlMethod => say(b"[ACPI] power button: reported through AML (PNP0C0C), which NONOS cannot run; use Power Off in the desktop's menu, or hold the button to force power off"),
        Button::HwReduced => say(b"[ACPI] power button: hardware-reduced platform, reported through AML, which NONOS cannot run; use Power Off in the desktop's menu"),
        Button::Unreachable => say(b"[ACPI] power button: its status register is not an I/O port NONOS reads; use Power Off in the desktop's menu"),
    }
}
