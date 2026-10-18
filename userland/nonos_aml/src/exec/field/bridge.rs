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

//! Whether a device is a PCI host bridge, and the integers its children
//! evaluate to.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use alloc::vec::Vec;

/// PNP0A03 (PCI) and PNP0A08 (PCI Express) host bridges, as the compressed
/// EISA ids firmware writes with EISAID() (section 6.1.5).
const HOST_BRIDGES: [u64; 2] = [0x030A_D041, 0x080A_D041];

impl<H: Host> Machine<'_, H> {
    pub(crate) fn is_host_bridge(
        &mut self,
        aml: &mut Aml,
        dev: Id,
        at: u32,
    ) -> Result<bool, Fault> {
        for seg in [*b"_HID", *b"_CID"] {
            let Some(id) = aml.ns.child(dev, seg) else { continue };
            let v = self.read_named(aml, id, Vec::new(), at)?;
            let v = self.deref(aml, v, at)?;
            let hit = match &v {
                Object::Integer(x) => HOST_BRIDGES.contains(x),
                Object::String(s) => matches!(s.borrow().as_slice(), b"PNP0A03" | b"PNP0A08"),
                Object::Package(p) => p
                    .borrow()
                    .iter()
                    .any(|o| matches!(o, Object::Integer(x) if HOST_BRIDGES.contains(x))),
                _ => false,
            };
            if hit {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// The integer a child object of `dev` evaluates to, if it has one.
    pub(crate) fn child_int(
        &mut self,
        aml: &mut Aml,
        dev: Id,
        seg: [u8; 4],
        at: u32,
    ) -> Result<Option<u64>, Fault> {
        let Some(id) = aml.ns.child(dev, seg) else { return Ok(None) };
        let v = self.read_named(aml, id, Vec::new(), at)?;
        let w = self.width(aml, 0);
        self.deref(aml, v, at)?
            .as_integer(w)
            .map(Some)
            .ok_or_else(|| self.fault(FaultKind::Type, at))
    }
}
