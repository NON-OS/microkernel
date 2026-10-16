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

//! `pkg install` and `pkg remove`, made jobs on a worker thread.

use alloc::boxed::Box;

use super::Verdict;
use crate::command::builtin::nox::pkg;
use crate::jobs::work::JobWork;
use crate::term::state::State;

/*
 * `pkg install` and `pkg remove`, typed bare or after `nox`: one installer
 * call each that can take up to 30 s, made on a worker thread so the window
 * paints and Ctrl+C is read meanwhile. `pkg status`, a usage error, and a
 * request no worker could be started for run on the ordinary dispatch.
 */
pub(super) fn package(state: &mut State, args: &[&[u8]]) -> Option<Verdict> {
    let args = match args {
        [b"nox", rest @ ..] if !rest.is_empty() => rest,
        _ => args,
    };
    let [b"pkg", rest @ ..] = args else {
        return None;
    };
    match pkg::prepare(state, rest) {
        pkg::Prepared::Job(job) => Some(Verdict::Job(JobWork::Pkg(Box::new(job)))),
        pkg::Prepared::Refused => Some(Verdict::Handled),
        pkg::Prepared::Inline => None,
    }
}
