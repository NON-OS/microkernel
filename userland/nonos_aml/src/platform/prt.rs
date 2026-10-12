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

//! PCI interrupt routing (ACPI 6.5, 6.2.13): each _PRT entry maps a slot's
//! INTA-INTD to a GSI, directly when its Source is 0, or through a link
//! device's current resources. Read after \_PIC, since the answer often
//! depends on the interrupt model.

use super::eval::{child, integer, Eval};
use super::prt_link::link_gsi;
pub use super::route::Route;
use crate::exec::{Fault, FaultKind};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::{Object, Ref};
use alloc::vec::Vec;

fn bad() -> Fault {
    Fault::new(FaultKind::Type, 0, 0)
}

/// The routes under `bridge`'s _PRT, in table order.
pub fn routes(aml: &mut Aml, eval: &mut Eval<'_>, bridge: Id) -> Option<Result<Vec<Route>, Fault>> {
    let prt = child(aml, eval, bridge, *b"_PRT", Vec::new())?;
    Some(prt.and_then(|p| {
        let Object::Package(p) = p else { return Err(bad()) };
        let entries: Vec<Object> = p.borrow().clone();
        let mut out = Vec::new();
        for e in entries {
            let Object::Package(e) = e else { return Err(bad()) };
            let e = e.borrow().clone();
            let field = |i: usize| e.get(i).and_then(integer).ok_or_else(bad);
            let (address, pin, index) = (field(0)?, field(1)?, field(3)?);
            let (slot, pin) = ((address >> 16) as u16, (pin & 3) as u8);
            let route = match e.get(2) {
                Some(Object::Integer(0)) => {
                    Route { slot, pin, link: None, gsi: u32::try_from(index).ok() }
                }
                Some(Object::Ref(Ref::Named(link, _))) | Some(Object::Named(link, _)) => {
                    let gsi = link_gsi(aml, eval, *link, index)?;
                    Route { slot, pin, link: Some(*link), gsi }
                }
                _ => return Err(bad()),
            };
            out.push(route);
        }
        Ok(out)
    }))
}
