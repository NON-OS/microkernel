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

/*
 * Whether a stream's owner still runs. A stream whose owner is gone is
 * taken over by the next process that begins one, from where it was left.
 */

use crate::process::ProcessState;

pub(super) fn alive(pid: u32) -> bool {
    crate::process::get_process(pid).is_some_and(|p| {
        !matches!(*p.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    })
}
