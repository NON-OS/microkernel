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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::node_ref::NodeRef;

impl<'a> Fdt<'a> {
    /// The node a phandle names. Zero and all-ones are reserved and name
    /// nothing; a phandle no node carries is `NoSuchNode`, since whoever
    /// quoted it expected a node there.
    pub fn by_phandle(&self, phandle: u32) -> Result<NodeRef<'a>, FdtError> {
        if phandle == 0 || phandle == u32::MAX {
            return Err(FdtError::NoSuchNode);
        }
        for node in self.nodes() {
            let node = node?;
            for prop in self.properties(&node) {
                let prop = prop?;
                if (prop.name == b"phandle" || prop.name == b"linux,phandle")
                    && prop.data.len() == 4
                    && prop.u32()? == phandle
                {
                    return Ok(node);
                }
            }
        }
        Err(FdtError::NoSuchNode)
    }
}
