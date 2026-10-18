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

//! A Notify on the UCSI device reads the CCI (at the storm's rate); each
//! pass of the loop reads the connectors it named, and the log says which
//! ports changed and when the session stuck, recovered or gave up.

use super::ppm::AcpiPpm;
use super::say::{changed, state};
use super::state::TypeC;
use crate::boot::Acpi;
use crate::host::CapsuleHost;
use crate::sys::clock::now_ms;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

pub fn notified(t: &mut Option<TypeC>, aml: &mut Aml, host: &mut CapsuleHost, id: Id) -> bool {
    let Some(t) = t.as_mut().filter(|t| t.dev == id) else { return false };
    let mut ppm = AcpiPpm::new(aml, host, t);
    let storms = t.session.storm.storms;
    t.session.notified(&mut ppm, now_ms());
    if t.session.storm.storms != storms {
        state(b"notification storm: notifications set aside, every port read after");
    }
    true
}

pub fn tick(acpi: &mut Acpi) {
    let Acpi { aml, host, typec, .. } = acpi;
    let Some(t) = typec.as_mut() else { return };
    let before = t.session.state;
    let mut ppm = AcpiPpm::new(aml, host, t);
    t.session.tick(&mut ppm, now_ms());
    if t.session.state != before {
        super::say::transition(t.session.state);
    }
    let bits = t.session.take_changed();
    for (i, p) in t.session.ports.iter().enumerate() {
        if bits & (1u128 << (i + 1)) != 0 {
            changed(i + 1, p);
        }
    }
}
