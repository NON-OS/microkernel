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

//! No frame on a protected link goes out in the clear, whenever the chip
//! restarts: at every radio call from the first probe through the 4-way
//! handshake and the session, the chip loses all it was given and lies
//! dead a while. The station seals every data frame itself, so whatever
//! the chip lost, the air sees only protected data and the handshake's
//! own EAPOL; a link the restart broke is joined afresh, never resumed.

use super::restart_rig::{connect, seen, send_ip, station};

/// Radio calls a join and a few seconds of session take with no restart.
fn calls_without_a_restart() -> u64 {
    let mut st = station(0);
    assert_eq!(connect(&mut st), 0);
    run(&mut st);
    st.radio().unwrap().calls
}

fn run(st: &mut super::restart_rig::Restarted) {
    for _ in 0..12 {
        send_ip(st);
        (0..25).for_each(|_| st.tick());
    }
}

#[test]
fn every_data_frame_stays_protected_across_a_restart_at_any_call() {
    let total = calls_without_a_restart();
    assert!(total > 100, "{total} calls");
    let (mut sealed, mut rejoined) = (0, 0);
    for at in 1..=total {
        let mut st = station(at);
        if connect(&mut st) != 0 || st.session().is_none() {
            rejoined += 1;
            connect(&mut st);
        }
        run(&mut st);
        let (n, clear) = seen(&st.radio().unwrap().air.sent);
        assert!(clear.is_empty(), "a restart at call {at} let out {:02x?}", clear[0]);
        sealed += n;
    }
    assert!(sealed > total as usize, "data went out, sealed: {sealed}");
    assert!(rejoined > 0, "some restarts broke a join");
}
