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

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::follow::Follow;

#[path = "job_drain.rs"]
mod job_drain;
#[path = "job_follow.rs"]
mod job_follow;

// One bounded slice of an install: while the installer is still loading,
// one look for the program it loads (`follow.rs`); then draining the loaded
// capsule's stdout into the terminal window, one `mk_proc_output` read, or,
// once the child has exited, the final flush of whatever is still buffered.
// Holds the progress cursor (elapsed start, whether any output has been
// seen) between slices.
pub struct InstallJob {
    /// Zero while the load is still being followed.
    pub(crate) pid: u32,
    stem: Vec<u8>,
    load: Option<Follow>,
    start: i64,
    saw_output: bool,
    asked_window: bool,
}

impl InstallJob {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            stem: Vec::new(),
            load: None,
            start: mk_time_millis(),
            saw_output: false,
            asked_window: false,
        }
    }

    /// A load the installer had not answered within the issue's wait.
    pub(super) fn loading(stem: &[u8], follow: Follow) -> Self {
        Self {
            pid: 0,
            stem: stem.to_vec(),
            load: Some(follow),
            start: mk_time_millis(),
            saw_output: false,
            asked_window: false,
        }
    }
}
