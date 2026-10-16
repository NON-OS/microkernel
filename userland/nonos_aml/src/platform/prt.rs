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

use super::eval::{child, Eval};
use super::prt_link::link_gsi;
pub use super::route::{entry, Route, Source};
use crate::exec::{Fault, FaultKind};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
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
            let (slot, pin, source) = entry(&e.borrow()).ok_or_else(bad)?;
            out.push(match source {
                Source::Gsi(gsi) => Route { slot, pin, link: None, gsi },
                Source::Link(link, index) => {
                    let gsi = link_gsi(aml, eval, link, index)?;
                    Route { slot, pin, link: Some(link), gsi }
                }
            });
        }
        Ok(out)
    }))
}
