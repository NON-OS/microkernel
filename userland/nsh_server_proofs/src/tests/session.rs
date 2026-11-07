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

//! The session: the client's frames reach the host as events, each held to
//! its body's rules; the host's output, commands and refused calls reach
//! the record; and the shell is stopped for exactly the reasons the record
//! can name.

use super::good::{check, received};
use crate::kit::run::Run;
use nsh_client::Verdict;
use nsh_server::Event as S;
use nsh_wire::Signal;

pub(super) fn ended(run: &Run) -> u8 {
    match check(run, &received(run), &[]).expect("the record checks").verdict {
        Verdict::Complete { end_reason, .. } => end_reason,
        v => panic!("{v:?}"),
    }
}

#[test]
fn input_window_and_signals_reach_the_host() {
    let mut run = Run::logged_in();
    run.user(|c| c.window(120, 40)).expect("window");
    run.user(|c| c.signal(Signal::Interrupt)).expect("signal");
    run.user(|c| c.signal(Signal::End)).expect("signal");
    let tail = &run.s_events[run.s_events.len() - 3..];
    assert_eq!(
        tail,
        [S::Window { cols: 120, rows: 40 }, S::Signal(Signal::Interrupt), S::Signal(Signal::End)]
    );
}
