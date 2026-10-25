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

//! A dock that docks and undocks, and the device that leaves with it.

use super::rig::machine;
use crate::consts::at;
use crate::hotplug::{dependents, is_dock, present};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn a_dock_docks_undocks_and_names_what_leaves_with_it() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let (dock, usb) = (at(&aml, "\\_SB.PCI0.DOCK").unwrap(), at(&aml, "\\_SB.PCI0.USB0").unwrap());
    assert!(is_dock(&aml, dock) && !is_dock(&aml, usb));
    assert_eq!(present(&mut aml, &mut evaluator(&mut probe), dock), Some(false));
    assert_eq!(dock_fn(&mut aml, &mut probe, dock, true), Some(Ok(true)));
    assert_eq!(present(&mut aml, &mut evaluator(&mut probe), dock), Some(true));
    assert_eq!(dependents(&mut aml, &mut evaluator(&mut probe), dock), [usb]);
    assert_eq!(dock_fn(&mut aml, &mut probe, dock, false), Some(Ok(true)));
    assert_eq!(present(&mut aml, &mut evaluator(&mut probe), dock), Some(false));
    assert_eq!(dock_fn(&mut aml, &mut probe, usb, true), None, "no _DCK");
}

fn dock_fn(
    aml: &mut crate::load::Aml,
    probe: &mut Probe,
    id: crate::namespace::Id,
    on: bool,
) -> Option<Result<bool, crate::exec::Fault>> {
    crate::hotplug::dock(aml, &mut evaluator(probe), id, on)
}
