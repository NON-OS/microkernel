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

use super::good::check_alone;
use crate::kit::raw_run::RawRun;
use nsh_client::Verdict;
use nsh_server::{EndReason, Event as S, Why};
use nsh_wire::Kind;

#[test]
fn a_body_the_rules_refuse_ends_the_session_and_the_record_says_aborted() {
    for inner in [
        vec![Kind::Input.byte()],
        vec![Kind::Window.byte(), 0, 0, 1, 0],
        vec![Kind::Signal.byte(), 3],
        vec![Kind::AuthOk.byte()],
        vec![0x99],
    ] {
        let mut r = RawRun::session();
        let f = r.raw.seal_inner(&inner);
        let frames = r.frames(&f);
        assert_eq!(frames, [(0x7F, vec![1, 0])], "{inner:?}");
        assert!(r.events.contains(&S::Closed(EndReason::Aborted)));
        let th = r.server.th_server_hello().expect("th");
        let rec: Vec<u8> = r
            .events
            .iter()
            .filter_map(|e| match e {
                S::Record(b) => Some(b.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat();
        let v = check_alone(&th, &rec, &[]).expect("the record is signed").verdict;
        assert!(matches!(v, Verdict::Complete { end_reason: 5, .. }), "{v:?}");
    }
}

#[test]
fn the_client_abort_ends_the_session_unanswered() {
    let mut r = RawRun::session();
    let f = r.raw.seal(Kind::Abort, &[3, 0]);
    assert!(r.send(&f).is_empty());
    assert!(r.events.contains(&S::Closed(EndReason::Aborted)));
    assert!(r.refused().contains(&Why::Aborted(nsh_wire::Reason::Policy)));
}
