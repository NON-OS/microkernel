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
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::K;
use nsh_roles::CapSet;
use nsh_server::{CallError, Why};
use nsh_wire::{AuthOk, Kind, WireError};

#[test]
fn a_changed_byte_is_not_answered() {
    let mut r = RawRun::proved();
    let ua = r.raw.user_auth(&GrantB::default(), K::User);
    let mut f = r.raw.seal(Kind::UserAuth, &ua);
    f[40] ^= 1;
    assert!(r.send(&f).is_empty());
    assert_eq!(r.refused(), [Why::Open]);
}

#[test]
fn nothing_may_arrive_while_the_host_starts_the_session() {
    let mut r = RawRun::proved();
    let ua = r.raw.user_auth(&GrantB::default(), K::User);
    let f = r.raw.seal(Kind::UserAuth, &ua);
    assert!(r.send(&f).is_empty());
    let again = r.raw.seal(Kind::UserAuth, &ua);
    assert_eq!(told(&r.frames(&again)), [PROTOCOL]);
    assert_eq!(r.refused(), [Why::Wire(WireError::Order)]);
}

#[test]
fn begin_names_no_more_than_the_grant() {
    let (mut r, caps) = admitted();
    let wider = CapSet::named(&caps_of(&["CoreExec", "IPC", "Memory", "Network"])).expect("caps");
    assert_eq!(r.server.begin(wider, r.now, &mut dry), Err(CallError::Beyond));
    assert_eq!(r.server.begin(CapSet::default(), r.now, &mut dry), Err(CallError::Beyond));
    let fewer = CapSet::named(&caps_of(&["CoreExec", "IPC"])).expect("caps");
    assert!(fewer.within(caps) && fewer != caps);
    let rng = &mut r.rng;
    r.server.begin(fewer, r.now, &mut |b| rng.fill(b)).expect("begin");
    let out = r.server.take_output();
    let frames = r.raw.open(&out);
    let ok = AuthOk::parse(&frames[0].1).expect("AUTH_OK");
    assert_eq!(ok.caps.iter().collect::<Vec<_>>(), [&b"CoreExec"[..], b"IPC"]);
    assert_eq!(ok.idle_s, 600);
    assert_eq!(r.server.begin(fewer, r.now, &mut dry), Err(CallError::NotNow), "once");
}
