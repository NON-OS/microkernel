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

//! Every kind of value a property may hold, packages nested to the limit
//! and no further, and what no property may hold dropped.

use super::rig::camera;
use crate::consts::at;
use crate::props::{properties, Value, MAX_DEPTH};
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn integers_strings_references_and_packages() {
    let (mut aml, _) = boot(&camera(), 2);
    let cam = at(&aml, "\\_SB.CAM0").unwrap();
    let mut probe = crate::probe::Probe::default();
    let p = properties(&mut aml, &mut evaluator(&mut probe), cam);
    let names: Vec<&[u8]> = p.iter().map(|p| p.name.as_slice()).collect();
    assert_eq!(names, [&b"rotation"[..], b"compatible", b"clock", b"reset-gpios", b"too-deep"]);
    assert_eq!(p[0].value, Value::Integer(180));
    assert_eq!(p[1].value, Value::String(b"sony,imx219".to_vec()));
    assert_eq!(p[2].value, Value::Reference(at(&aml, "\\_SB.CLK0").unwrap()));
    let gpio = Value::Reference(at(&aml, "\\_SB.GPO0").unwrap());
    let (three, zero, one) = (Value::Integer(3), Value::Integer(0), Value::Integer(1));
    let want = Value::Package(vec![Value::Package(vec![gpio, three, zero, one])]);
    assert_eq!(p[3].value, want, "a GPIO list: a package in a package");
}

#[test]
fn packages_nest_to_the_limit() {
    let (mut aml, _) = boot(&camera(), 2);
    let cam = at(&aml, "\\_SB.CAM0").unwrap();
    let mut probe = crate::probe::Probe::default();
    let p = properties(&mut aml, &mut evaluator(&mut probe), cam);
    let mut v = &p[4].value;
    let mut levels = 0;
    while let Value::Package(inner) = v {
        levels += 1;
        match inner.first() {
            Some(next) => v = next,
            None => break,
        }
    }
    assert_eq!(levels, MAX_DEPTH, "six levels written, four kept, the rest emptied");
    assert!(matches!(v, Value::Package(e) if e.is_empty()));
}
