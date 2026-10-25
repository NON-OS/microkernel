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
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// A child of `/cpus` that describes a CPU, not a cache or a cluster map.
/// Older trees omit `device_type`, so the node name stands in for it there.
pub(super) fn is_cpu(fdt: &Fdt, node: &NodeRef) -> Result<bool, FdtError> {
    Ok(match fdt.property(node, b"device_type")? {
        Some(kind) => kind.string() == Ok(b"cpu"),
        None => node.base_name() == b"cpu",
    })
}
