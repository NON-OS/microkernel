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

//! Handing a processor's tables to the kernel, and saying when it refused.

use super::record::record;
use super::state::Cpu;
use crate::sys::acpi_cpu::acpi_cpu;
use crate::sys::log::Line;

pub fn send(cpu: &Cpu) {
    if let Err(e) = acpi_cpu(&record(cpu.uid, &cpu.tables)) {
        Line::new(b"cpu ")
            .dec(cpu.uid)
            .str(b": the kernel refused its tables: ")
            .dec(e.unsigned_abs())
            .end();
    }
}
