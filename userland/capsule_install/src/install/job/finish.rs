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

//! Closing a job: the outcome the done or failed screen shows, built from
//! the receipt when there is one and from the counters when there is not.

use alloc::string::String;

use crate::install::state::{Outcome, Screen, State};

pub fn finish(state: &mut State, error: Option<String>) {
    let Some(job) = state.job.take() else { return };
    let (disk_guid, partition_guid, bytes_written, store_files) = match &job.receipt {
        Some(r) => (r.disk_guid.text(), r.esp_guid().text(), r.bytes_written, r.store_files),
        None => ([b'-'; 36], [b'-'; 36], job.done, 0),
    };
    state.screen = if error.is_some() { Screen::Failed } else { Screen::Done };
    state.outcome = Some(Outcome {
        disk_guid,
        partition_guid,
        bytes_written,
        bytes_verified: job.done,
        store_files,
        seconds: job.write_seconds,
        error,
    });
}
