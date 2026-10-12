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

//! The hierarchical data extension: a device's subnodes by path and by
//! reference, their own children, a loop read once, a link to nothing
//! skipped, and a device with no _DSD an empty tree.

use super::rig::camera;
use crate::consts::at;
use crate::props::{tree, Value};
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn ports_and_endpoints_below_the_device() {
    let (mut aml, _) = boot(&camera(), 2);
    let cam = at(&aml, "\\_SB.CAM0").unwrap();
    let mut probe = crate::probe::Probe::default();
    let t = tree(&mut aml, &mut evaluator(&mut probe), cam);
    assert!(t.name.is_empty());
    assert_eq!(t.properties.len(), 5);
    let names: Vec<&[u8]> = t.children.iter().map(|c| c.name.as_slice()).collect();
    assert_eq!(names, [&b"port@0"[..], b"port@1"], "port@9 names nothing");
    let (p0, p1) = (&t.children[0], &t.children[1]);
    assert_eq!(p0.properties[0].value, Value::Integer(0));
    assert_eq!(p1.properties[0].value, Value::Integer(1), "linked by reference");
    assert_eq!(p0.children.len(), 1);
    let ep = &p0.children[0];
    assert_eq!(ep.name, b"endpoint@0");
    let lanes = Value::Package(vec![Value::Integer(1), Value::Integer(2)]);
    assert_eq!(ep.properties[0].value, lanes);
    assert!(ep.children.is_empty(), "the link back to its port is a loop, read once");
}

#[test]
fn no_dsd_is_an_empty_tree() {
    let (mut aml, _) = boot(&camera(), 2);
    let bare = at(&aml, "\\_SB.BARE").unwrap();
    let mut probe = crate::probe::Probe::default();
    let t = tree(&mut aml, &mut evaluator(&mut probe), bare);
    assert_eq!(t, Default::default());
}
