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

//! Whose launch MkAppLaunch admits, pure. Held in kernel_proofs (K.5).

/// How a launch was admitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Launch {
    /// AppInstall, or the desktop shell: any listing, the run no one's.
    Free,
    /// A capsule SERVES pairs with this listing: the run is noted as its
    /// process's and ends with it (capsule_linux/served_run.rs).
    Served,
    /// Neither: EPERM.
    Refused,
}

/// A launch of `listing` by a caller that is `free` (AppInstall, or the
/// shell), else that owns the endpoints `owns` answers for. `listings` is
/// SERVES' (registry/serves.rs LISTINGS): a capsule there owned by the caller
/// launches its own listing alone, so the window starts its core and nothing
/// else; whether that core is served is MkServeOne's, by its listing key.
pub(crate) fn admit_launch(
    free: bool,
    owns: impl Fn(&str) -> bool,
    listing: &str,
    listings: &[(&str, &str)],
) -> Launch {
    if free {
        return Launch::Free;
    }
    match listings.iter().any(|&(capsule, own)| owns(capsule) && own == listing) {
        true => Launch::Served,
        false => Launch::Refused,
    }
}
