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

use super::describe;
use crate::arch::aarch64::psci::{set_method, PsciMethod};
use crate::arch::fdt::find::psci::{find, Conduit};
use crate::arch::fdt::Fdt;
use crate::sys::serial;

/// The PSCI conduit, before anything powers a CPU on or the machine off.
/// Without one, CPU_ON, SYSTEM_OFF and SYSTEM_RESET answer NOT_SUPPORTED
/// instead of trapping, and the line says so.
pub(super) fn populate(fdt: &Fdt) -> Result<(), &'static [u8]> {
    let line: &[u8] = match find(fdt).map_err(describe)? {
        Some(Conduit::Smc) => {
            set_method(PsciMethod::Smc);
            b"[PSCI] firmware reached by SMC"
        }
        Some(Conduit::Hvc) => {
            set_method(PsciMethod::Hvc);
            b"[PSCI] firmware reached by HVC"
        }
        None => b"[PSCI] the device tree names none; no power-off or reset through it",
    };
    serial::println(line);
    Ok(())
}
