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

//! The wake GPEs: those a device's _PRW names (ACPI 6.5, 7.3.13). They are
//! armed only when a device is set to wake the machine, which NONOS does not
//! yet do, so they are never enabled at run time.

use crate::exec::{Fault, FaultKind};
use crate::load::Aml;
use crate::namespace::{Id, ROOT};
use crate::object::Object;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// Every _PRW object in the namespace, depth first in definition order.
fn prw_objects(aml: &Aml) -> Vec<Id> {
    let (mut out, mut stack) = (Vec::new(), alloc::vec![ROOT]);
    while let Some(id) = stack.pop() {
        let at = stack.len();
        for c in aml.ns.children(id) {
            match aml.ns.get(c) {
                Some(e) if e.seg == *b"_PRW" => out.push(c),
                Some(e) if e.kind.is_container() => stack.insert(at, c),
                _ => {}
            }
        }
    }
    out
}

/// The first element of _PRW is the event: an Integer in the FADT's blocks,
/// or a Package of a GPE block device and an index, which names an event
/// outside them and is skipped here.
fn event_of(v: &Object) -> Result<Option<u16>, FaultKind> {
    let Object::Package(p) = v else { return Err(FaultKind::Type) };
    match p.borrow().first() {
        Some(Object::Integer(n)) => u16::try_from(*n).map(Some).map_err(|_| FaultKind::Index),
        Some(Object::Package(_)) => Ok(None),
        _ => Err(FaultKind::Type),
    }
}

/// The wake events, and each _PRW that could not be read; a device whose
/// _PRW fails is one whose wake event stays unknown, not one that stops the
/// rest.
pub fn wake_gpes(
    aml: &mut Aml,
    eval: &mut dyn FnMut(&mut Aml, Id) -> Result<Object, Fault>,
) -> (BTreeSet<u16>, Vec<(Id, Fault)>) {
    let (mut wake, mut failed) = (BTreeSet::new(), Vec::new());
    for id in prw_objects(aml) {
        match eval(aml, id).map(|v| event_of(&v)) {
            Ok(Ok(Some(n))) => {
                wake.insert(n);
            }
            Ok(Ok(None)) => {}
            Ok(Err(kind)) => failed.push((id, Fault::new(kind, 0, 0))),
            Err(f) => failed.push((id, f)),
        }
    }
    (wake, failed)
}
