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

//! A program the store does not have, run by the built-in BusyBox.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::app::{bind_built_in, AppIdentity};

use super::exec_resolve::Program;

/*
 * Not in the store: one of the built-in BusyBox's programs runs as itself,
 * which is how a shell with no Linux tree behind it still runs `ls`. The
 * built-in is part of this capsule's own image, measured by its manifest,
 * and argv[0] says which program to be. Reached through a script, it runs
 * as that script's interpreter, in the script's box.
 */
pub fn built_in(
    path: &[u8],
    argv: Vec<Vec<u8>>,
    named: Option<AppIdentity>,
) -> Result<Program, u64> {
    if crate::linux::built_in::serves(path).is_none() {
        return Err(errno::fail(errno::ENOENT));
    }
    let bytes = crate::linux::built_in::built_in().bytes;
    let app = named.unwrap_or_else(|| bind_built_in(&bytes));
    Ok(Program { path: b"/bin/busybox".to_vec(), bytes, argv, app })
}
