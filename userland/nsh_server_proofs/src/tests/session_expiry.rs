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

use super::session::*;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::{HOUR, NOW};
use crate::kit::run::Run;
use nsh_server::{EndReason, Event as S};

#[test]
fn an_idle_session_is_stopped() {
    let mut run = Run::logged_in();
    run.now += 599_999;
    run.host(|s, now, r| s.tick(now, r));
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Stop(_))));
    run.user(|c| c.input(b"x")).expect("input");
    run.now += 599_999;
    run.host(|s, now, r| s.tick(now, r));
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Stop(_))), "input resets the clock");
    run.now += 1;
    run.host(|s, now, r| s.tick(now, r));
    assert_eq!(run.s_events.last(), Some(&S::Stop(EndReason::Idle)));
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert_eq!(ended(&run), 3);
}

#[test]
fn the_session_ends_with_its_grant() {
    let short = GrantB { not_after_ms: NOW + HOUR / 12, ..GrantB::default() };
    let mut run = Run::new(&short, Table::shipped());
    run.pump();
    run.now = NOW + HOUR / 12 - 1;
    run.host(|s, now, r| s.tick(now, r));
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Stop(_))));
    run.user(|c| c.input(b"x")).expect("input");
    run.now += 1;
    run.host(|s, now, r| s.tick(now, r));
    assert_eq!(run.s_events.last(), Some(&S::Stop(EndReason::Expired)));
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert_eq!(ended(&run), 4);
}
