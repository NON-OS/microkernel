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

use super::good::{kinds, received};
use super::session::*;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::BASE_ID;
use crate::kit::run::Run;
use nsh_client::Event as C;
use nsh_server::{EndReason, Event as S};
use nsh_wire::EntryBody;

#[test]
fn a_grant_revoked_mid_session_stops_it() {
    let table = Table::shipped();
    let handle = table.clone();
    let mut run = Run::new(&GrantB::default(), table);
    run.pump();
    handle.install(&RevB { ids: vec![BASE_ID], ..RevB::default() });
    run.host(|s, now, r| s.tick(now, r));
    assert_eq!(run.s_events.last(), Some(&S::Stop(EndReason::Host)));
    run.host(|s, now, r| s.exit(-1, now, r)).expect("exit");
    assert_eq!(ended(&run), 6);
}

#[test]
fn output_is_cut_into_frames_and_counted_in_the_record() {
    let mut run = Run::logged_in();
    let big = vec![b'a'; 16_384 + 10];
    run.host(|s, now, r| s.output(&big, now, r)).expect("output");
    run.host(|s, now, r| s.output(b"tail", now, r)).expect("output");
    let outs: Vec<usize> = run
        .c_events
        .iter()
        .filter_map(|e| match e {
            C::Output(b) => Some(b.len()),
            _ => None,
        })
        .collect();
    assert_eq!(outs, [16_384, 10, 4]);
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    let all = [&big[..], b"tail"].concat();
    let want = EntryBody::Output { bytes: all.len() as u64, digest: &nonos_hash::sha256(&all) };
    assert_eq!(kinds(&rec), [want, EntryBody::Exit(0)]);
}
