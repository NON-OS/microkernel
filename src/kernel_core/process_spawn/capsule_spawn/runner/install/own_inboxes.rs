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

//! The inboxes a capsule owns from its first moment: `proc.<pid>`, where its
//! requests arrive, and `stdin.<pid>`, what its parent feeds it.

extern crate alloc;

use super::super::super::spec::SpawnError;
use crate::ipc::nonos_inbox;
use alloc::format;

/// Messages a capsule's stdin inbox holds before its parent is told EBUSY.
const STDIN_CAPACITY: usize = 64;

pub(super) fn register(pid: u32) -> Result<(), SpawnError> {
    nonos_inbox::register_inbox(&format!("proc.{}", pid), pid)
        .map_err(|_| SpawnError::ProcessCreation)?;
    /*
     * What its parent feeds it (MkProcInput), drained with MkStdinRead. It
     * goes with `proc.<pid>` when the process is torn down. Small: every
     * capsule has one, and a parent refused by a full one feeds it later.
     */
    nonos_inbox::register_inbox_with_capacity(&format!("stdin.{}", pid), pid, STDIN_CAPACITY)
        .map_err(|_| SpawnError::ProcessCreation)
}
