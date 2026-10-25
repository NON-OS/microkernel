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

//! The commands that wait on the network, made stepped jobs.

use alloc::boxed::Box;

use super::Verdict;
use crate::command::builtin::git::clone;
use crate::command::builtin::nox::http;
use crate::jobs::work::JobWork;
use crate::term::state::State;

/*
 * The commands that wait on the network: `http` (and its other names) and
 * `git clone`, typed bare or after `nox`. Each becomes a stepped job, so the
 * window paints and Ctrl+C is read while it runs.
 */
pub(super) fn network(state: &mut State, args: &[&[u8]]) -> Option<Verdict> {
    let args = match args {
        [b"nox", rest @ ..] if !rest.is_empty() => rest,
        _ => args,
    };
    let work = match args {
        [b"http" | b"curl" | b"get" | b"fetch", rest @ ..] => {
            http::prepare(state, rest).map(|job| JobWork::Http(Box::new(job)))
        }
        [b"git", b"clone", rest @ ..] => {
            clone::prepare(state, rest).map(|job| JobWork::GitClone(Box::new(job)))
        }
        _ => return None,
    };
    Some(match work {
        Some(work) => Verdict::Job(work),
        None => {
            state.last_status = 1;
            Verdict::Handled
        }
    })
}
