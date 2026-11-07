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

//! The frames around the login that the real client never sends, and the
//! host's answer to an admitted login: `AUTH_OK` names what the session
//! capsule was given, never more than the grant, and a host that cannot
//! start the session says `BUSY`.

use super::admit::*;
use crate::kit::raw_run::RawRun;
use crate::kit::rng::dry;
use nsh_roles::CapSet;
use nsh_server::{CallError, Event, Why};
use nsh_wire::Reason;

#[test]
fn a_host_that_cannot_start_the_session_says_busy() {
    let (mut r, _) = admitted();
    r.server.deny(r.now, &mut dry).expect("deny");
    let out = r.server.take_output();
    assert_eq!(told(&r.raw.open(&out)), [(0x7F, Reason::Busy.code().to_le_bytes())]);
    assert!(r.server.is_over());
    assert_eq!(r.server.deny(r.now, &mut dry), Err(CallError::NotNow));
    let (mut r, caps) = admitted();
    r.server.begin(caps, r.now, &mut dry).expect("begin is answered");
    assert!(r.server.take_output().is_empty(), "no session id, no AUTH_OK");
    assert_eq!(r.server.events().last(), Some(&Event::Refused(Why::Rand)));
}

#[test]
fn begin_and_deny_wait_for_a_login() {
    let mut r = RawRun::proved();
    let caps = CapSet::named(&caps_of(&["CoreExec"])).expect("caps");
    assert_eq!(r.server.begin(caps, r.now, &mut dry), Err(CallError::NotNow));
    assert_eq!(r.server.deny(r.now, &mut dry), Err(CallError::NotNow));
}
