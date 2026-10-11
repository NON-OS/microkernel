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

//! What the installer pins a package to (plan/25). An escrow package is
//! delivered encrypted and checked chunk by chunk against its tree, so its
//! chunk root and length are the pin and its package hash is not; any other
//! package is pinned by its hash.

use nonos_marketplace_abi::CapsuleRelease;

pub fn pinned(release: &CapsuleRelease, escrow: bool) -> bool {
    if !escrow {
        return release.package_hash.iter().any(|&b| b != 0);
    }
    release.ext.as_ref().is_some_and(|x| x.chunk_root != [0; 32] && x.content_len != 0)
}
