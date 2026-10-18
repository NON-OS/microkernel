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

//! How much one definition block may take. A table past any of these is
//! refused whole, so a hostile SSDT cannot exhaust the capsule's memory or
//! its stack before the first method runs.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Largest table accepted, header included.
    pub max_table: usize,
    /// Deepest nesting of objects inside objects; the parser recurses once
    /// per level, so this bounds its stack.
    pub max_depth: u32,
    /// Most nodes one arena holds, method bodies parsed later included.
    pub max_nodes: usize,
}

impl Limits {
    /// Room for the largest DSDTs firmware ships (a few hundred KiB, a few
    /// hundred thousand objects) with margin, and a depth no compiler's output
    /// reaches but a recursion-crafted table would.
    pub const DEFAULT: Limits = Limits { max_table: 4 << 20, max_depth: 64, max_nodes: 1 << 20 };
}

impl Default for Limits {
    fn default() -> Self {
        Self::DEFAULT
    }
}
