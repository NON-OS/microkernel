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
 * Starting the model fetcher for `qwen get` or `qwen tiers` as this tab's
 * foreground job: its lines are this screen, Ctrl-C stops it, and the
 * kernel keeps how far a download came for the next `qwen get`.
 */

use nonos_libc::{mk_time_millis, mk_tool_run};

use super::fetch_check::{kept, refusal};
use super::fetch_words::{request, Fetch};
use crate::jobs::{submit, JobWork, StdinQueue};
use crate::term::state::State;

/* errno, its name, and what it means for the fetcher. */
const REASONS: &[(i64, &[u8], &[u8])] = &[
    (-2, b"ENOENT", b"this system was built without the model fetcher"),
    (-1, b"EPERM", b"the model fetcher's signed artifacts did not verify"),
    (-16, b"EBUSY", b"another qwen get or qwen tiers is running; let it end or stop it"),
    (-12, b"ENOMEM", b"no room to start the model fetcher"),
    (-38, b"ENOSYS", b"this kernel cannot start programs by name"),
    (-13, b"EACCES", b"Linux and Qwen are off for this boot, at setup or by Safe Mode or Recovery"),
    (-100, b"ENETDOWN", b"this boot runs no network (Air-Gapped, Safe Mode or Recovery)"),
];

/* Run `fetch`; history keeps only its checked words, never the line as typed. */
pub fn enter(state: &mut State, fetch: Fetch<'_>) {
    let line = kept(&fetch);
    state.history.push(&line);
    if let Some(work) = start(state, &fetch) {
        let _ = submit(state, &line, false, work);
        state.fg_running = true;
        state.fg_started_ms = mk_time_millis();
    }
}

/* The fetcher as a job, or `None` with the reason on screen. */
pub fn start(state: &mut State, fetch: &Fetch<'_>) -> Option<JobWork> {
    let refuse = |state: &mut State, line: &[u8]| {
        state.scrollback.push_error(line);
        state.last_status = 1;
        None
    };
    if let Some(line) = refusal(fetch) {
        return refuse(state, &line);
    }
    let rc = mk_tool_run(b"tool.model-fetch", &request(fetch));
    let Some(pid) = u32::try_from(rc).ok().filter(|&p| p != 0) else {
        let (name, why) = REASONS.iter().find(|(e, _, _)| *e == rc).map_or(
            (&b"refused"[..], &b"the kernel would not start the model fetcher"[..]),
            |(_, n, w)| (*n, *w),
        );
        return refuse(state, &[&b"qwen: "[..], why, b" (", name, b")"].concat());
    };
    crate::jobs::tty::attach(state, pid);
    state.last_status = 0;
    Some(JobWork::ExternalStage { pid, stdin: StdinQueue::new(false), capture: None })
}
