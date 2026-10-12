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

//! Device properties from _DSD (ACPI 6.5, 6.2.5): the one reader every
//! caller uses. A device's properties under the device-properties UUID,
//! values of every kind a property may hold, and the data-only subnodes the
//! hierarchical data extension hangs below a device, as a tree.

mod link;
mod set;
mod tree;
mod uuid;
mod value;

pub use set::{properties, properties_under};
pub use tree::{tree, Node, MAX_NODES};
pub use uuid::{DEVICE_PROPERTIES_UUID, HIERARCHICAL_DATA_UUID};
pub use value::{Property, Value, MAX, MAX_DEPTH};
