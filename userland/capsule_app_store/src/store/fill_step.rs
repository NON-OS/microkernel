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

//! What one step of filling did, and how an answer is kept: a market that
//! did not reply is remembered as silent.

use super::fill_order::next;
use super::market::Failure;
use super::state::State;

/// How soon the next step runs while there is one: quick enough that the
/// pane fills as the cursor lands, without a busy loop.
pub const STEP_MS: i64 = 30;

/// What a step did.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Nothing left to ask.
    Done,
    /// Asked about a listing; the screen shows what changed.
    Shown,
    /// Asked about a listing nothing on screen depends on yet.
    Quiet,
}

/// The answer, noting when there was none at all.
pub(super) fn kept<T>(got: Result<T, Failure>, silent: &mut bool) -> Option<T> {
    *silent |= matches!(got, Err(Failure::NoReply));
    got.ok()
}

impl State {
    /// Whether the market still has something to be asked. False once it
    /// has stopped answering (`port` zero), since every answer would be the
    /// same silence.
    pub fn filling(&self) -> bool {
        self.port != 0 && next(&self.listings, self.current_index(), now_ms()).is_some()
    }
}

pub(super) fn now_ms() -> u64 {
    nonos_libc::mk_time_millis().max(0) as u64
}
