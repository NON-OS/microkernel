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

use crate::kit::raw_run::RawRun;
use crate::kit::rng::dry;
use crate::kit::run::Run;
use nsh_server::CallError;

#[test]
fn host_calls_wait_for_a_running_session() {
    let mut r = RawRun::proved();
    assert_eq!(r.server.output(b"x", r.now, &mut dry), Err(CallError::NotNow));
    assert_eq!(r.server.command(b"x", r.now, &mut dry), Err(CallError::NotNow));
    assert_eq!(r.server.exit(0, r.now, &mut dry), Err(CallError::NotNow));
    let mut run = Run::logged_in();
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert_eq!(run.host(|s, now, r| s.output(b"x", now, r)), Err(CallError::NotNow));
    assert_eq!(run.host(|s, now, r| s.exit(0, now, r)), Err(CallError::NotNow));
}
