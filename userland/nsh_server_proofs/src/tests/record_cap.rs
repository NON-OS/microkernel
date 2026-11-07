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

//! The record as the host keeps it: handed over as it grows, checkpointed,
//! held to its cap with room kept for its end, and signed by the host keys
//! or, if they fail, left to its checkpoints.

use super::good::{check, kinds, received};
use super::record::*;
use crate::kit::host::{config, Keys};
use nsh_client::{Event as C, Verdict};
use nsh_server::{Config, EndReason, Event as S, Why, MIN_RECORD};
use nsh_wire::EntryBody;

#[test]
fn a_record_at_its_cap_stops_the_shell_and_still_ends_signed() {
    let mut run = run_with(Config { max_record: MIN_RECORD, ..config() }, Keys::default());
    let line = vec![b'x'; 4000];
    let mut n = 0;
    while !run.s_events.contains(&S::Stop(EndReason::Host)) {
        run.host(|s, now, r| s.command(&line, now, r)).expect("command");
        run.host(|s, now, r| s.output(b"y", now, r)).expect("output");
        n += 1;
        assert!(n < 64, "the cap was never met");
    }
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    assert!(rec.len() <= MIN_RECORD, "{}", rec.len());
    let v = check(&run, &rec, &[]).expect("the record at its cap checks").verdict;
    assert!(matches!(v, Verdict::Complete { end_reason: 6, .. }), "{v:?}");
    assert_eq!(kinds(&rec).last(), Some(&EntryBody::Exit(0)));
    let outputs: u64 = kinds(&rec)
        .iter()
        .map(|k| match k {
            EntryBody::Output { bytes, .. } => *bytes,
            _ => 0,
        })
        .sum();
    assert_eq!(outputs, n, "every byte sent is counted");
}

#[test]
fn a_record_the_host_keys_cannot_sign_is_not_sent() {
    let keys = Keys::default();
    let mut run = run_with(Config { ckpt_every: 1, ..config() }, keys.clone());
    run.host(|s, now, r| s.command(b"ls", now, r)).expect("command");
    keys.refuse.set(true);
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert!(run.refused().contains(&Why::Sign));
    assert!(run.s_events.contains(&S::Closed(EndReason::Exit)));
    assert!(run.server.is_over());
    assert!(
        run.c_events.ends_with(&[C::Exit(0)]),
        "the client waits for a record that never comes"
    );
    assert_eq!(run.checkpoints().len(), 1, "the checkpoint signed before the keys failed");
}
