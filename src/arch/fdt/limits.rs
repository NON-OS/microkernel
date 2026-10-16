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

//! The fixed bounds every walk of a blob works within.

/// Largest blob accepted, our own cap: real trees sit far below it (recalled,
/// unverified), so a header claiming more is far likelier corrupt than real.
pub const MAX_BLOB: usize = 2 << 20;

/// Deepest node nesting accepted, the root at depth 0. Real trees stop around
/// six levels; the bound is what lets every per-depth table be fixed storage.
pub const MAX_DEPTH: usize = 16;

/// Most cells one `#address-cells`, `#size-cells` or `#interrupt-cells` may
/// name. PCI addresses are three and GIC specifiers at most four.
pub const MAX_CELLS: u32 = 4;

/// What the spec says a node without `#address-cells` / `#size-cells` uses.
pub const DEFAULT_ADDRESS_CELLS: u32 = 2;
pub const DEFAULT_SIZE_CELLS: u32 = 1;
