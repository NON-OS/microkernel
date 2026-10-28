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

//! What a process is handed after the last step of its spawn that can fail,
//! and before it is queued to run: its arguments, and for a terminal's run of
//! the Linux personality its reserved request. A process reads its arguments
//! before anything else (the personality sizes its heap from them), so set
//! once the spawn had returned, they raced its first instruction: on KVM an
//! install read none and ran in a program's heap, half an install's.

use super::params::InstallParams;

pub(super) fn admit(params: &InstallParams<'_>, pid: u32) {
    if let Some(argv) = &params.argv {
        let _ = crate::process::set_argv(pid, argv.clone());
    }
    crate::userspace::capsule_linux::admit_terminal_run(params.name, pid);
}
