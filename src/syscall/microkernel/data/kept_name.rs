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

//! The names a Linux app's kept home and kept installs have on the data
//! volume (docs/design/kept-homes-and-installs.md): a home's two slots
//! (`/h-`), the installs' index (`/li.`) and the kept packs (`/lp-`). Only
//! a caller holding ForeignExec, which only the Linux personality holds,
//! reads, sizes, brings in or removes one. Any other caller finds them
//! refused, whether or not they exist, so it can neither read a kept home
//! nor plant one an app would load, nor take one away.

/// What every kept name begins with.
pub(crate) const KEPT: [&[u8]; 3] = [b"/h-", b"/li.", b"/lp-"];

/// Whether `name` is a kept home's or a kept install's.
pub(crate) fn is_kept(name: &[u8]) -> bool {
    KEPT.iter().any(|prefix| name.starts_with(prefix))
}

/// Whether a caller may reach `name` at all: any other name, and a kept one
/// only when `foreign` says the caller holds ForeignExec. `foreign` is
/// asked only for a kept name.
pub(crate) fn may_reach(name: &[u8], foreign: impl FnOnce() -> bool) -> bool {
    !is_kept(name) || foreign()
}
