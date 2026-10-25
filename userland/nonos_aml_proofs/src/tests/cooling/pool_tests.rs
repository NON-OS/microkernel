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

//! Shared power resources: _ON_ for the first device to need one, _OFF for
//! the last to let go, and a failed _ON_ that leaves nothing held.

use super::pool_rig::src;
use crate::consts::at;
use crate::device_power::{to_d0, to_d3, PowerPool};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn shared_resources_turn_off_with_their_last_holder() {
    let (mut aml, _) = boot(&src(), 2);
    let dev = |aml: &crate::load::Aml, p| at(aml, p).unwrap();
    let (f0, f1, pr1) = (dev(&aml, "\\_SB.FAN0"), dev(&aml, "\\_SB.FAN1"), dev(&aml, "\\_SB.PR1"));
    let (mut probe, mut pool) = (Probe::default(), PowerPool::default());
    to_d0(&mut aml, &mut evaluator(&mut probe), &mut pool, f0).unwrap();
    to_d0(&mut aml, &mut evaluator(&mut probe), &mut pool, f1).unwrap();
    assert_eq!(pool.holders(pr1), 2);
    to_d3(&mut aml, &mut evaluator(&mut probe), &mut pool, f0).unwrap();
    to_d3(&mut aml, &mut evaluator(&mut probe), &mut pool, f1).unwrap();
    let want: Vec<String> =
        [0x11, 0x21, 0x40, 0x43, 0x20, 0x10].map(|n| format!("Integer({n})")).into();
    assert_eq!(probe.debug, want);
    assert_eq!(pool.holders(pr1), 0);
}

#[test]
fn a_failed_on_gives_back_what_it_took() {
    let (mut aml, _) = boot(&src(), 2);
    let (f2, pr1) = (at(&aml, "\\_SB.FAN2").unwrap(), at(&aml, "\\_SB.PR1").unwrap());
    let (mut probe, mut pool) = (Probe::default(), PowerPool::default());
    assert!(to_d0(&mut aml, &mut evaluator(&mut probe), &mut pool, f2).is_err());
    assert_eq!(pool.holders(pr1), 0);
    assert_eq!(probe.debug, ["Integer(17)", "Integer(16)"]);
    assert_eq!(probe.fatal, [(1, 2, 3)]);
}
