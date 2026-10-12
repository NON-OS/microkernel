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

//! Setting an output's brightness with _BCM (ACPI 6.5, B.6.3), to one of
//! its _BCL levels: from a brightness key, or a level asked over the
//! capsule's port, taken to the nearest level each output supports.

use super::state::{Backlight, Output};
use crate::boot::evaluate;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec;
use nonos_aml::exec::Fault;
use nonos_aml::load::Aml;
use nonos_aml::object::Object;
use nonos_aml::video::nearest;

pub fn set(o: &mut Output, aml: &mut Aml, host: &mut CapsuleHost, level: u8) -> Result<(), Fault> {
    if let Some(bcm) = aml.ns.child(o.id, *b"_BCM") {
        evaluate(aml, host, bcm, vec![Object::Integer(u64::from(level))])?;
    }
    o.current = Some(level);
    Ok(())
}

/// Every output to the level nearest `percent`; false when any refused.
pub fn set_all(b: &mut Backlight, aml: &mut Aml, host: &mut CapsuleHost, percent: u8) -> bool {
    let mut all = true;
    for o in b.outputs.iter_mut() {
        let level = nearest(&o.levels, percent);
        if set(o, aml, host, level).is_err() {
            all = false;
            Line::new(b"brightness: _BCM refused").end();
        }
    }
    if all && !b.outputs.is_empty() {
        Line::new(b"brightness ").dec(u64::from(percent.min(100))).str(b"% asked").end();
    }
    all
}
