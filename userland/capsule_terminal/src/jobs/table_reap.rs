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

//! Taking the finished jobs out of the table, with what each ended as.

use alloc::vec::Vec;

use super::{JobState, JobTable};

impl JobTable {
    pub fn drop_done(&mut self) -> Vec<(u32, Vec<u8>, i32, bool)> {
        let mut done = Vec::new();
        self.jobs.retain(|j| {
            if j.state == JobState::Done {
                done.push((j.id, j.cmdline.clone(), j.status, j.background));
                false
            } else {
                true
            }
        });
        done
    }
}
