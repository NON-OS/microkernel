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

//! The event layer's registers through MkAcpiPort. The PM1, GPE and
//! SMI_CMD ports are granted at bring-up (boot/sci.rs); any other port is
//! refused by the kernel and reads as a failure here.

use crate::sys::acpi::{port_read, port_write};
use crate::sys::clock::sleep_ms;
use nonos_aml::events::EventIo;

pub struct PortIo;

impl EventIo for PortIo {
    fn read(&mut self, port: u16, width: u8) -> Option<u16> {
        port_read(port as u64, width).ok().map(|v| v as u16)
    }

    fn write(&mut self, port: u16, width: u8, value: u16) -> bool {
        port_write(port as u64, width, value as u32).is_ok()
    }

    fn sleep(&mut self, ms: u64) {
        sleep_ms(ms)
    }
}
