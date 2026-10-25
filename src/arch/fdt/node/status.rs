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
    /// A node is in service unless its `status` says otherwise. Anything but
    /// "okay" or the older "ok" ("disabled", "fail", "reserved") is not ours
    /// to touch.
    pub fn is_enabled(&self, node: &NodeRef) -> Result<bool, FdtError> {
        match self.property(node, b"status")? {
            None => Ok(true),
            Some(status) => Ok(matches!(status.string(), Ok(b"okay") | Ok(b"ok"))),
        }
    }
}
