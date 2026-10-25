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

//! A USB host's ports as the namespace lays them out (ACPI 6.5, 9.14): the
//! host's root hub, its ports (each with _ADR the port number), and the
//! ports of any hub firmware describes behind one. A port is a device with
//! _UPC or _PLD; the walk goes three levels below the host and no further.

use super::pld::pld;
use super::port::UsbPort;
use super::upc::upc;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::{child, integer, Eval};
use alloc::vec::Vec;

const DEPTH: u32 = 3;

fn walk(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    at: Id,
    parent: u8,
    depth: u32,
    out: &mut Vec<UsbPort>,
) {
    if depth == 0 {
        return;
    }
    let children: Vec<Id> = aml.ns.children(at).collect();
    for c in children {
        if !aml.ns.get(c).is_some_and(|e| e.kind.is_container()) {
            continue;
        }
        let u =
            child(aml, eval, c, *b"_UPC", Vec::new()).and_then(|r| r.ok()).as_ref().and_then(upc);
        let p =
            child(aml, eval, c, *b"_PLD", Vec::new()).and_then(|r| r.ok()).as_ref().and_then(pld);
        let adr = child(aml, eval, c, *b"_ADR", Vec::new()).and_then(|r| r.ok());
        let adr = adr.as_ref().and_then(integer).and_then(|a| u8::try_from(a).ok());
        // A port by having _UPC or _PLD, whether or not they read: a port
        // with a malformed _PLD is a port firmware says too little about.
        let is_port = [*b"_UPC", *b"_PLD"].iter().any(|s| aml.ns.child(c, *s).is_some());
        if is_port {
            out.push(UsbPort { id: c, parent, port: adr.unwrap_or(0), upc: u, pld: p });
        }
        let below = if is_port { adr.unwrap_or(0) } else { parent };
        walk(aml, eval, c, below, depth - 1, out);
    }
}

pub fn usb_ports(aml: &mut Aml, eval: &mut Eval<'_>, host: Id) -> Vec<UsbPort> {
    let mut out = Vec::new();
    walk(aml, eval, host, 0, DEPTH, &mut out);
    out
}
