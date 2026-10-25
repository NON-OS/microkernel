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

use super::kind::MemoryType;

#[derive(Debug, Clone, Copy)]
pub struct PageAttributes {
    pub memory_type: MemoryType,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
    pub global: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub contiguous: bool,
    /// GP: indirect branches into this page must land on a BTI landing pad.
    /// Meaningful only for code EL1 runs, and only with FEAT_BTI.
    pub guarded: bool,
}

impl PageAttributes {
    pub const fn new(
        memory_type: MemoryType,
        write: bool,
        execute: bool,
        user: bool,
        global: bool,
        accessed: bool,
        dirty: bool,
    ) -> Self {
        Self {
            memory_type,
            write,
            execute,
            user,
            global,
            accessed,
            dirty,
            contiguous: false,
            guarded: false,
        }
    }
}

impl PageAttributes {
    /// The same mapping, guarded or not. The caller decides, from FEAT_BTI
    /// and from whether the code it maps carries landing pads; bit 50 is RES0
    /// on a core without the feature.
    pub const fn with_guard(mut self, guarded: bool) -> Self {
        self.guarded = guarded;
        self
    }
}

impl Default for PageAttributes {
    fn default() -> Self {
        Self::kernel_data()
    }
}
