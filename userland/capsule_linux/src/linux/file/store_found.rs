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

//! Whether the store has a file at a key, told apart from not being able
//! to say: a caller that treats absence as permission must never treat a
//! failure the same way.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::root::Key;

/// Ok(true) when the file is there, in the store's tree or a pack (a pack
/// holds no proof name: install/proof_names.rs), Ok(false) only when the
/// store says it is not, and the store's reason otherwise.
pub fn found(at: &Key) -> Result<bool, &'static str> {
    let got = vfs::lookup(mk_getpid(), at.as_bytes())?;
    Ok(got.is_some() || super::store::packed_found(at))
}
