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

//! The last step of a clone: the pack indexed and the work tree checked out.

use alloc::format;

use nonos_git::{clone_into, CloneRequest, ObjectId, TransportError};

use super::{CloneJob, DEPTH};
use crate::command::output::Output;
use crate::jobs::JobProgress;

impl CloneJob {
    /// One step, the last: the pack is indexed and the work tree checked
    /// out. It is the store's work and the CPU's, with nothing left to wait
    /// on the network for.
    pub(super) fn write(
        &mut self,
        out: &mut Output<'_>,
        head: ObjectId,
        pack: &[u8],
    ) -> JobProgress {
        let git_dir = format!("{}/.git", self.into);
        let work_tree = format!("{}/", self.into);
        let request = CloneRequest {
            git_dir: &git_dir,
            work_tree: &work_tree,
            head,
            branch: &self.branch,
            shallow: DEPTH > 0,
            url: Some(&self.url),
        };
        match clone_into(&mut self.storage, &request, pack) {
            Ok(files) => {
                let line =
                    format!("Cloned into {}, {files} files, {}", self.into, self.route.name());
                out.writeln(line.as_bytes());
                JobProgress::Done(0)
            }
            Err(_) => self.fail(out, TransportError::Malformed),
        }
    }
}
