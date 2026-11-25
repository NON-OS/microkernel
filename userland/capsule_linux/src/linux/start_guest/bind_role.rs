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

//! The binding a launch chooses its role on, before its guest exists: the
//! program's userspace hash and the policy beside it, held to the policy's
//! own local trailer (app::bind). No proof of the program is asked here. The
//! kernel would absorb it into no family, since none exists before the
//! spawn; the start proves the program after the spawn (prove.rs).

use super::super::app::{bind, bind_built_in, AppIdentity};
use super::super::origin::Origin;

/// The identity `bytes` at `path` will run under, for choosing its role.
pub(in crate::linux) fn bind_role(
    path: &[u8],
    bytes: &[u8],
    origin: &Origin,
) -> Result<AppIdentity, &'static str> {
    match origin {
        Origin::BuiltIn => Ok(bind_built_in(bytes)),
        Origin::Store => bind(path, bytes),
    }
}
