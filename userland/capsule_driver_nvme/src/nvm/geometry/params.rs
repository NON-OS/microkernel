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

//! The namespace parameters an I/O queue is built around.

/// The namespace parameters an I/O queue is built around, bundled so bring_up
/// stays within the argument limit.
pub struct NamespaceGeometry {
    pub nsid: u32,
    pub capacity_sectors: u64,
    pub lba_size: u32,
    /// Blocks one command may move: what the data buffer holds, and no more
    /// than the controller's MDTS allows.
    pub max_sectors: u32,
}
