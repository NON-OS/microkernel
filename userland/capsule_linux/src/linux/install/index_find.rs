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

use alloc::{string::String, vec::Vec};

use super::index::{Index, Pkg};

/// The needs not met yet, and those that wait (next_need).
pub type Needs<'a> = (&'a mut Vec<String>, &'a mut Vec<String>);

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

    /// Whether a package of `done` has or provides `want`.
    pub fn provided_in(&self, want: &str, done: &[String]) -> bool {
        let (list, key) = want.strip_prefix("so:").map_or((&self.names, want), |l| (&self.libs, l));
        list.iter().any(|(k, i)| k == key && done.contains(&self.pkgs[*i].name))
    }

    /// The next need, and whether it is the listed package. One named only by
    /// what provides it (`icu-data`) waits in `later` while named needs remain,
    /// as one may name the provider it takes (`icu-data-full`). One met is passed.
    pub fn next_need(&self, (wanted, later): Needs, done: &[String]) -> Option<(String, bool)> {
        while let Some(w) = wanted.pop() {
            if done.is_empty() {
                return Some((w, true));
            } else if self.provided_in(&w, done) {
            } else if w.starts_with("so:") || self.listed(&w).is_some() {
                return Some((w, false));
            } else {
                later.push(w);
            }
        }
        Some((core::iter::from_fn(|| later.pop()).find(|w| !self.provided_in(w, done))?, false))
    }
}
