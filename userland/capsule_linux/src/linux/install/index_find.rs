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

//! Finding a package in the index: by a library it provides, by a name it
//! has or provides, and, for the package the market listed, by its own name
//! only.

use super::index::{Index, Pkg};

impl Index {
    /// What an install asks for next: a library by its soname, the listed
    /// package (`listed`, the first ask) by its own name, a need by name.
    pub fn resolve(&self, want: &str, listed: bool) -> Option<&Pkg> {
        match want.strip_prefix("so:") {
            Some(lib) => self.by_lib(lib),
            None if listed => self.listed(want),
            None => self.by_name(want),
        }
    }

    pub fn by_lib(&self, soname: &str) -> Option<&Pkg> {
        self.libs.iter().find(|(k, _)| k == soname).and_then(|(_, i)| self.pkgs.get(*i))
    }

    pub fn by_name(&self, name: &str) -> Option<&Pkg> {
        self.names.iter().find(|(k, _)| k == name).and_then(|(_, i)| self.pkgs.get(*i))
    }

    /// The package of exactly this name, never one that only provides it:
    /// the market's pin names the listed package's bytes, not a provider's.
    pub fn listed(&self, name: &str) -> Option<&Pkg> {
        self.pkgs.iter().find(|p| p.name == name)
    }
}
