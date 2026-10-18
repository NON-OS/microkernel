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

//! S3 in the specification's order (ACPI 6.5, 7.4 and 16.1): every method
//! the sequence runs and every register it writes, on one timeline, and
//! only the policy's wake devices armed, each for a state its _PRW reaches.

use super::logged::logged;
use super::model::Model;
use super::order::S3_ORDER;
use super::rig::{laptop, policy, VECTOR};
use crate::object::Object;
use crate::rig::Rig;
use crate::suspend::{suspend_s3, Skip};

#[test]
fn every_step_in_order() {
    let mut r = laptop(true);
    let wake = policy(&r);
    let mut hw = Model::default();
    let log = hw.log.clone();
    let out = {
        let Rig { aml, probe } = &mut r;
        suspend_s3(aml, &mut logged(probe, log.clone()), &mut hw, &wake, VECTOR)
    };

    assert_eq!(*log.borrow(), S3_ORDER);
    assert!(out.entered && out.error.is_none());
    assert_eq!(out.armed, [wake[0], wake[1]]);
    assert_eq!(out.skipped, [(wake[2], Skip::TooShallow(1)), (wake[3], Skip::NoPrw)]);
    assert_eq!(out.s0.map(|t| (t.a, t.b)), Some((0, 0)));
    assert!(matches!(out.wak, Some(Ok(Object::Package(_)))));
    let asleep: Vec<u16> = hw.armed_asleep.unwrap().into_iter().collect();
    assert_eq!(
        asleep,
        [0x6D, 0x6E],
        "the sleep button's GPE stays off: the policy does not name it"
    );
    assert!(hw.armed.is_empty(), "every GPE disarmed on the way back");
}
