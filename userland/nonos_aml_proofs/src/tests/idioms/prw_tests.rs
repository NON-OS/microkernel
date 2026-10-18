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

//! _PRW in the shapes firmware writes it (ACPI 6.5, 7.3.13): a Name with
//! the GPE and sleep state and a power resource; a Method that returns what
//! a shared helper builds from its arguments; and the GPE block form,
//! whose event lies outside the FADT blocks and is passed over.

use super::prw_rig::rig;
use crate::events::wake_gpes;
use crate::exec::{ExecLimits, Machine};
use crate::object::Object;
use crate::rig::Rig;

#[test]
fn wake_events_from_names_methods_and_blocks() {
    let mut r = rig();
    let Rig { aml, probe } = &mut r;
    let mut eval = |aml: &mut crate::load::Aml, id| {
        Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, vec![])
    };
    let (wake, failed) = wake_gpes(aml, &mut eval);
    assert_eq!(wake.into_iter().collect::<Vec<_>>(), [0x6D, 0x6E]);
    assert!(failed.is_empty(), "{failed:?}");
}

#[test]
fn the_power_resource_named_in_prw_is_reachable() {
    let mut r = rig();
    let v = r.call("\\_SB.XHCI._PRW", vec![]).unwrap();
    let Object::Package(p) = v else { panic!("{v:?}") };
    let third = p.borrow()[2].clone();
    assert!(
        !matches!(third, Object::Integer(_) | Object::Uninit),
        "a name, not a number: {third:?}"
    );
    assert_eq!(r.num("\\_SB.PWRR._STA"), 1);
}
