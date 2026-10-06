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

use alloc::string::String;

/* One declaration as parsed. The flags say, once per parse, what the value
 * holds, so the cascade skips the substitution pass for the many values
 * that name no custom property and no light-dark() pair. */
pub struct Decl {
    pub name: String,
    pub value: String,
    /* Declared !important: it outranks every normal declaration. */
    pub important: bool,
    pub flags: u8,
}

impl Decl {
    /* The value holds var(). */
    pub const VAR: u8 = 1;
    /* The value holds light-dark(). */
    pub const LIGHT_DARK: u8 = 2;
    /* The name is a custom property, --name. */
    pub const CUSTOM: u8 = 4;

    pub fn new(name: String, value: String, important: bool) -> Decl {
        let mut flags = 0;
        if value.contains("var(") {
            flags |= Decl::VAR;
        }
        if value.contains("light-dark(") {
            flags |= Decl::LIGHT_DARK;
        }
        if name.starts_with("--") {
            flags |= Decl::CUSTOM;
        }
        Decl { name, value, important, flags }
    }

    /* The value needs substitution before an applier may read it. */
    pub fn needs_resolve(&self) -> bool {
        self.flags & (Decl::VAR | Decl::LIGHT_DARK) != 0
    }

    /* Heap and inline bytes this declaration keeps: the parse budget's
     * measure. Each allocation is counted at 16 bytes at least. */
    pub fn cost(&self) -> usize {
        core::mem::size_of::<Decl>() + self.name.len().max(16) + self.value.len().max(16)
    }
}
