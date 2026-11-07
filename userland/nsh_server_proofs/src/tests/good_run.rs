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

//! The good run: the real client accepts the host on the real `NONOSATT`
//! checker, the role table admits the user, a command runs, and the record
//! the host signed checks offline.

use crate::kit::run::Run;
use nsh_client::{Event as C, Verdict};
use nsh_server::{EndReason, Event as S};

/// The document the host sent in this run's `HOST_PROOF`.
use super::good::*;

#[test]
fn a_command_runs_and_the_record_checks() {
    let mut run = Run::logged_in();
    assert!(matches!(run.s_events[0], S::Admit(_)), "{:?}", run.s_events);
    assert!(run.s_events.iter().any(|e| matches!(e, S::Started { .. })));
    run.user(|c| c.input(b"echo hi\n")).expect("input");
    assert!(run.s_events.contains(&S::Input(b"echo hi\n".to_vec())));
    run.host(|s, now, r| s.command(b"echo hi", now, r)).expect("command");
    run.host(|s, now, r| s.output(b"hi\n", now, r)).expect("output");
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert_eq!(run.c_err, None);
    let tail: Vec<_> =
        run.c_events.iter().filter(|e| !matches!(e, C::Record(_))).cloned().collect();
    assert_eq!(tail[tail.len() - 3..], [C::Output(b"hi\n".to_vec()), C::Exit(0), C::Closed]);
    assert!(run.s_events.contains(&S::Closed(EndReason::Exit)));
    assert!(run.server.is_over() && run.client.is_over());
    let record = received(&run);
    assert_eq!(record, run.kept_record(), "the host keeps the record it sends");
    let report = check(&run, &record, &[]).expect("the record checks");
    let want = Verdict::Complete { entries: 3, end_ms: run.now, end_reason: 2, boxed: false };
    assert_eq!(report.verdict, want);
}
