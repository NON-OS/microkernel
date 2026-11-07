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

use super::good::received;
use super::session::*;
use crate::kit::raw_run::RawRun;
use crate::kit::run::Run;
use nsh_client::Event as C;
use nsh_server::{EndReason, Event as S, Why};
use nsh_wire::{Kind, WireError};

#[test]
fn close_stops_the_shell_and_the_record_says_close() {
    let mut run = Run::logged_in();
    run.user(|c| c.close()).expect("close");
    assert_eq!(run.s_events.last(), Some(&S::Stop(EndReason::Close)));
    run.host(|s, now, r| s.exit(130, now, r)).expect("exit");
    assert!(run.c_events.ends_with(&[C::Exit(130), C::Record(received(&run)), C::Closed]));
    assert_eq!(ended(&run), 1);
}

#[test]
fn nothing_may_follow_close() {
    let mut r = RawRun::session();
    let close = r.raw.seal(Kind::Close, &[]);
    assert!(r.send(&close).is_empty());
    let input = r.raw.seal(Kind::Input, b"x");
    assert_eq!(r.frames(&input), [(0x7F, vec![1, 0])]);
    assert_eq!(r.refused(), [Why::Wire(WireError::Order)]);
}

#[test]
fn the_shell_exiting_on_its_own_says_exit() {
    let mut run = Run::logged_in();
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert_eq!(ended(&run), 2);
}
