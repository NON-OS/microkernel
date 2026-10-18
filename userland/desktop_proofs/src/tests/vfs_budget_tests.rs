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

//! How long the desktop waits on the file service (vfs_client/constants.rs).
//! A change the person asked for waited 300 ms, the budget of the desktop's
//! own once-a-second look, and on a slow laptop's disk the change was made
//! after the desktop had said "the file service did not answer".

use crate::vfs_constants::{
    budget, OP_CLOSE, OP_GENERATION, OP_LIST, OP_MKDIR, OP_OPEN, OP_RENAME, OP_RMDIR,
    OP_STORE_STATUS, OP_UNLINK, TIMEOUT_MS, WRITE_TIMEOUT_MS,
};

/// What `mk_ipc_call` waits when given no budget (kernel sys_ipc_call.rs).
const KERNEL_DEFAULT_MS: u64 = 5_000;

#[test]
fn a_change_the_person_asked_for_waits_as_long_as_the_kernel_would() {
    for op in [OP_OPEN, OP_CLOSE, OP_MKDIR, OP_UNLINK, OP_RENAME, OP_RMDIR] {
        assert_eq!(budget(op), WRITE_TIMEOUT_MS, "op {op}");
    }
    assert_eq!(WRITE_TIMEOUT_MS, KERNEL_DEFAULT_MS);
}

/// The looks the runner makes on its own stay short: the shell's frame loop
/// waits on them.
#[test]
fn the_desktops_own_looks_stay_short() {
    for op in [OP_LIST, OP_GENERATION, OP_STORE_STATUS] {
        assert_eq!(budget(op), TIMEOUT_MS, "op {op}");
    }
    assert_eq!(TIMEOUT_MS, 300);
}
