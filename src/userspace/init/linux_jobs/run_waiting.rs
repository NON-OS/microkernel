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

//! Whether a run is queued and not yet started. Until init starts it, the run
//! role is not registered, so run_busy says no run is going; this keeps a
//! second press of the store's Open from queueing a copy.

use super::queue::{Job, PENDING};

/// True while a run waits in the queue.
pub(crate) fn run_waiting() -> bool {
    PENDING.lock().iter().any(|j| matches!(j, Job::Run(..)))
}
