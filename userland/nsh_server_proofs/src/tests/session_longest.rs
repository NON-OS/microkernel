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

//! nsh.front lets go of a connection at its own bound, with no end and no
//! record, so the server must end every session it can admit before that
//! bound: a role at the longest span a table may give, held by a grant as
//! long, is stopped by the server at its end, inside nsh.front's bound
//! counted from the connection, and the record names the end.

use super::front_limits::{HANDSHAKE_MS, SESSION_MAX_MS, STOP_GRACE_MS};
use super::session::ended;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::table::TableB;
use crate::kit::roles_build::NOW;
use crate::kit::run::Run;
use nsh_roles::MAX_SPAN_MS;
use nsh_server::{EndReason, Event as S};

const _: () = assert!(
    HANDSHAKE_MS + MAX_SPAN_MS < SESSION_MAX_MS,
    "a session admitted at the handshake bound ends inside nsh.front's bound"
);
const _: () = assert!(
    SESSION_MAX_MS - HANDSHAKE_MS - MAX_SPAN_MS == STOP_GRACE_MS,
    "what is left after the longest session is the stop's grace"
);

fn longest() -> Run {
    let mut table = TableB::default();
    table.role("operator").max_session_ms = MAX_SPAN_MS;
    let grant = GrantB { not_after_ms: NOW + 2 * MAX_SPAN_MS, ..GrantB::default() };
    let mut run = Run::new(&grant, Table::new(&table, &RevB::default()));
    run.pump();
    assert!(run.c_events.iter().any(|e| matches!(e, nsh_client::Event::LoggedIn(_))));
    run
}

#[test]
fn the_longest_role_is_stopped_by_the_server_at_its_end() {
    let mut run = longest();
    let end = NOW + MAX_SPAN_MS;
    run.now = end - 1;
    run.user(|c| c.input(b"x")).expect("input");
    run.host(|s, now, r| s.tick(now, r));
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Stop(_))), "held to its end");
    run.now = end;
    run.host(|s, now, r| s.tick(now, r));
    assert_eq!(run.s_events.last(), Some(&S::Stop(EndReason::Expired)));
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    assert_eq!(ended(&run), EndReason::Expired.code());
    assert!(run.now - NOW < SESSION_MAX_MS, "the record goes out before nsh.front lets go");
}

/// The bounds nsh.front opens every connection's server with are ones the
/// server keeps, with the enrolment the machine holds, and its relays to
/// nsh.attest are bounded inside the handshake.
#[test]
fn nsh_fronts_bounds_are_ones_the_server_keeps() {
    use super::front_limits::{HANDSHAKE_MS, IDLE_S, MAX_RECORD, RELAY_MS};
    let (_, files) = super::devkit_host::made_at(super::devkit_host::AT_SPEC);
    let enrol = super::devkit_host::file(&files, "enrol").to_vec();
    let cfg = nsh_server::Config {
        enrol,
        handshake_ms: HANDSHAKE_MS,
        idle_s: IDLE_S,
        ckpt_every: 0,
        max_record: MAX_RECORD,
    };
    assert!(cfg.valid());
    assert!((nsh_server::MIN_RECORD..=nsh_wire::record::MAX_RECORD).contains(&MAX_RECORD));
    const { assert!(IDLE_S >= 1 && IDLE_S <= nsh_wire::MAX_IDLE_S) };
    // A relay to nsh.attest that never answers is given up on well inside
    // the handshake, so the handshake still ends on time.
    const { assert!(RELAY_MS > 5_000 && 2 * RELAY_MS <= HANDSHAKE_MS) };
}
