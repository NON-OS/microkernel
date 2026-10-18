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

//! Starting and stopping a fan: _FSL with its level, or the D0 and D3
//! moves with their shared power resources counted.

use super::state::{Drive, Fan};
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use alloc::vec;
use nonos_aml::device_power::{to_d0, to_d3, PowerPool};
use nonos_aml::exec::{Fault, FaultKind};
use nonos_aml::load::Aml;
use nonos_aml::object::Object;

pub fn run_fan(
    fan: &Fan,
    pool: &mut PowerPool,
    aml: &mut Aml,
    host: &mut CapsuleHost,
    on: bool,
) -> Result<(), Fault> {
    let mut ev = evaluator(host);
    match fan.drive {
        Drive::Level { on: hi, off: lo } => {
            let fsl =
                aml.ns.child(fan.id, *b"_FSL").ok_or(Fault::new(FaultKind::Undefined, 0, 0))?;
            ev(aml, fsl, vec![Object::Integer(if on { hi } else { lo })]).map(|_| ())
        }
        Drive::Power if on => to_d0(aml, &mut ev, pool, fan.id),
        Drive::Power => to_d3(aml, &mut ev, pool, fan.id),
    }
}
