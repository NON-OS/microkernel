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

//! A processor's _OSC (ACPI 6.5, 6.2.11 and 8.4): the operating system
//! says which processor-control features it implements. The capability
//! bits are the processor vendor's, not ACPI's; firmware often loads its
//! _PSS and _CST tables only from inside this call, so it runs first.

use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::{negotiate, Eval, OscError};

/// 4077A616-290C-47BE-9EBD-D87058713953, Intel's processor _OSC UUID, in
/// ToUUID's byte order.
pub const PROCESSOR_UUID: [u8; 16] = [
    0x16, 0xA6, 0x77, 0x40, 0x0C, 0x29, 0xBE, 0x47, 0x9E, 0xBD, 0xD8, 0x70, 0x58, 0x71, 0x39, 0x53,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessorOsc {
    /// The processor defines no _OSC.
    Absent,
    /// Firmware's answer: the capability word as it granted it.
    Granted(u32),
    Refused(OscError),
}

/// Query, then commit, `capabilities` (revision 1, one capability word).
pub fn processor_osc(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    cpu: Id,
    capabilities: u32,
) -> ProcessorOsc {
    let Some(osc) = aml.ns.child(cpu, *b"_OSC") else { return ProcessorOsc::Absent };
    match negotiate(aml, eval, osc, (&PROCESSOR_UUID, 1), &[0, capabilities]) {
        Ok(w) => ProcessorOsc::Granted(w[1]),
        Err(e) => ProcessorOsc::Refused(e),
    }
}
