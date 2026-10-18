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

//! A device's _DEP read from the namespace: a reference, a path string
//! resolved from the device's scope, and a string naming nothing kept as
//! dangling; a device with no _DEP depends on nothing.

use crate::consts::at;
use crate::deps::{dependencies, Dep};
use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn machine() -> Vec<u8> {
    let dep = package(3, &[name("\\_SB.PMIC"), string("GPO0"), string("\\_SB.GONE")]);
    cat(&[
        &device("\\_SB.PMIC", &[]),
        &device("\\_SB.GPO0", &[]),
        &device("\\_SB.CAM0", &name_obj("_DEP", &dep)),
        &device("\\_SB.TPD0", &[]),
    ])
}

#[test]
fn references_paths_and_dangling() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let cam = at(&aml, "\\_SB.CAM0").unwrap();
    let got = dependencies(&mut aml, &mut evaluator(&mut probe), cam);
    let want = [
        Dep::On(at(&aml, "\\_SB.PMIC").unwrap()),
        Dep::On(at(&aml, "\\_SB.GPO0").unwrap()),
        Dep::Dangling,
    ];
    assert_eq!(got, want);
    let tpd = at(&aml, "\\_SB.TPD0").unwrap();
    assert_eq!(dependencies(&mut aml, &mut evaluator(&mut probe), tpd), []);
}
