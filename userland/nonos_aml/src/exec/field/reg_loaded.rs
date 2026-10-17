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

//! _REG for regions a table brings once their space is already connected
//! (section 6.5.4).

use crate::exec::{Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin};
use crate::region::Space;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// After a table loads: its regions in a space already connected are
    /// announced too, or a device it defines would wait for a _REG that
    /// never comes. A fault there does not undo the load; it is counted
    /// for the capsule to say.
    pub(crate) fn announce_loaded(&mut self, aml: &mut Aml) {
        let mut spaces: Vec<u8> = Vec::new();
        for (_, space) in regions(aml) {
            if aml.rt.connected & Space(space).bit() != 0 && !spaces.contains(&space) {
                spaces.push(space);
            }
        }
        for space in spaces {
            let failed = self.announce(aml, Space(space)).len() as u32;
            aml.rt.reg_faults = aml.rt.reg_faults.saturating_add(failed);
        }
    }
}

/// Every region in the namespace: its scope and its space, in definition
/// order.
pub(super) fn regions(aml: &Aml) -> Vec<(Id, u8)> {
    let mut out = Vec::new();
    for id in 0..aml.ns.len() as Id {
        let Some(e) = aml.ns.get(id) else { continue };
        if let (Kind::OpRegion, Origin::Table { table, node }) = (e.kind, e.origin) {
            out.push((e.parent, aml.imm(table, aml.kid(table, node, 1)) as u8));
        }
    }
    out
}
