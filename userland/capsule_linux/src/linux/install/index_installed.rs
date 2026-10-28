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

//! A need named only by what provides it, met by a provider an earlier
//! install placed, where there is one. icu-data-en and icu-data-full both
//! provide `icu-data` and both hold icudt74l.dat: with KeePassXC's
//! icu-data-full installed, Telegram's closure took the index's first
//! provider, icu-data-en, and its install was refused that file. The
//! provider taken stays in the closure, so the app's record names it and an
//! uninstall of the first app leaves it. A need a package has as its own
//! name is resolved as before: one named explicitly is the one taken.

use alloc::string::String;

use super::{Index, Pkg};

impl Index {
    /// As `resolve`, preferring for a provided need a provider in
    /// `installed`, the packages earlier installs placed.
    pub fn resolve_installed(
        &self,
        want: &str,
        listed: bool,
        installed: &[String],
    ) -> Option<&Pkg> {
        if listed || self.listed(want).is_some() {
            return self.resolve(want, listed);
        }
        let (list, key) = want.strip_prefix("so:").map_or((&self.names, want), |l| (&self.libs, l));
        let mut providers =
            list.iter().filter(|(k, _)| k == key).filter_map(|(_, i)| self.pkgs.get(*i));
        providers.find(|p| installed.contains(&p.name)).or_else(|| self.resolve(want, false))
    }
}
