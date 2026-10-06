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

//! Starting Qwen on this terminal. The question travels on the program's
//! stdin as its first line; it is never an argument, never the job's name
//! and never history, which keep only `qwen` and the tier.

use alloc::vec::Vec;

use nonos_libc::mk_tool_run;

use crate::jobs::{JobWork, StdinQueue};
use crate::term::state::State;

/// Start `tier` as this terminal's child, attached to its screen, with
/// `question` (if any) queued as its first line. `None`, with the reason on
/// screen, when the kernel refuses.
pub fn spawn(state: &mut State, tier: &'static [u8], question: &[u8]) -> Option<JobWork> {
    let rc = mk_tool_run(b"tool.qwen", tier);
    let pid = match u32::try_from(rc) {
        Ok(pid) if pid != 0 => pid,
        _ => {
            super::refused::refused(state, tier, rc);
            return None;
        }
    };
    crate::jobs::tty::attach(state, pid);
    let mut stdin = StdinQueue::new(true);
    if !question.is_empty() {
        let mut first = Vec::with_capacity(question.len() + 1);
        first.extend_from_slice(question);
        first.push(b'\n');
        let _ = stdin.push(&first);
    }
    Some(JobWork::ExternalStage { pid, stdin, capture: None })
}
