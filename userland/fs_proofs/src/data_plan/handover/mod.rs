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

//! The handover record (fs/blockfs_volume/handover/), its layout and its
//! seal, mounted as they ship beside the key header they sit in.

#[path = "../../../../../src/fs/blockfs_volume/handover/layout.rs"]
mod layout;
#[path = "../../../../../src/fs/blockfs_volume/handover/place.rs"]
mod place;
#[path = "../../../../../src/fs/blockfs_volume/handover/seal.rs"]
mod seal;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_moved;
