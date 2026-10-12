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

//! An _OSI ladder as firmware writes one (ACPI 6.5, 5.7.2's _OSI example,
//! at the length real tables reach): thirty probes, Windows years mixed
//! with vendor and feature strings, each setting the OS year only if
//! answered, so the year is the newest Windows string NONOS answers.

use super::more::*;
use super::run::ok;
use crate::enc::*;
use crate::exec::WINDOWS;
use crate::rig::Rig;

const OTHERS: &[&str] = &[
    "Linux",
    "Darwin",
    "FreeBSD",
    "Module Device",
    "Processor Device",
    "3.0 Thermal Model",
    "3.0 _SCP Extensions",
    "Processor Aggregator Device",
    "Extended Address Space Descriptor",
];

#[test]
fn the_ladder_climbs_to_the_newest_answered_year() {
    let mut body = store(&int(0), &name("\\OSYS"));
    let mut probes = 0;
    for (year, s) in WINDOWS.iter().enumerate() {
        let probe = invoke("\\_OSI", &[string(core::str::from_utf8(s).unwrap())]);
        body.extend(if_(&probe, &store(&int(2000 + year as u64), &name("\\OSYS"))));
        probes += 1;
    }
    for s in OTHERS {
        body.extend(if_(&invoke("\\_OSI", &[string(s)]), &store(&int(1), &name("\\OSYS"))));
        probes += 1;
    }
    body.extend(ret(&name("\\OSYS")));
    let defs = cat(&[&name_obj("\\OSYS", &int(0)), &method("\\INIO", 0, &body)]);
    let mut r = Rig::new(&defs).connected();
    let (v, ran) = ok(&mut r, "\\INIO");
    assert_eq!(v, 2000 + WINDOWS.len() as u64 - 1, "no vendor or feature string answered");
    assert!(probes >= 30);
    assert!(ran.steps < 400, "{} steps for {probes} probes", ran.steps);
}
