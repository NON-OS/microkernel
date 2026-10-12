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

//! SET_POWER_CAP (20, meter u8, milliwatts u32): the meter's hardware cap
//! through _SHL, within the bounds its _PMC gives. EPERM while the switch
//! is off, or when the meter's cap cannot be set by the OS; ENOENT without
//! that meter or its _PMC; EINVAL for a short request or a cap out of
//! range; EIO when firmware fails.

use super::switch::WRITES;
use crate::boot::{evaluator, Acpi};
use crate::server::protocol::{EINVAL, EIO, ENOENT, EPERM};
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::power_meter::{meters, read_meter, set_cap, CapError};

pub fn set_power_cap(acpi: &mut Acpi, req: &[u8]) -> (i32, Vec<u8>) {
    if !WRITES {
        Line::new(b"set_power_cap refused: the switch is off").end();
        return (EPERM, Vec::new());
    }
    let Some(&[meter, a, b, c, d]) = req.get(1..6) else { return (EINVAL, Vec::new()) };
    let Some(dev) = meters(&acpi.devices).get(usize::from(meter)).copied() else {
        return (ENOENT, Vec::new());
    };
    let Acpi { aml, host, .. } = acpi;
    let Some(pmc) = read_meter(aml, &mut evaluator(host), dev).pmc else {
        return (ENOENT, Vec::new());
    };
    let code = match set_cap(aml, &mut evaluator(host), dev, &pmc, u32::from_le_bytes([a, b, c, d]))
    {
        Ok(()) => 0,
        Err(CapError::NotConfigurable) => EPERM,
        Err(CapError::OutOfRange) => EINVAL,
        Err(CapError::Failed) => EIO,
    };
    (code, Vec::new())
}
