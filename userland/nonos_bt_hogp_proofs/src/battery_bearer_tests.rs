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

//! The bearer's own bounds while the battery client runs: the shared
//! request budget, which the battery yields, and the ATT timeout, which
//! ends the bearer for every client on it (Vol 3, Part F, 3.3.3).

use crate::battery_refuse_tests::{still_types, KEY};
use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{keyboard_mouse, Db, Peripheral, Run};
use nonos_bt_hogp::host::{Battery, Outcome, MAX_REQUESTS, TIMEOUT_MS};
use nonos_bt_hogp::report::Input;
use nonos_bt_hogp::HogpError;
use nonos_bt_wire::att::opcode::READ_REQ;

#[test]
fn a_spent_request_budget_stops_the_battery_not_the_keyboard() {
    let mut db = Db::default()
        .service(0x1812)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .service(0x180f)
        .char(READ | NOTIFY, 0x2a19, &[90]);
    // Five descriptors fit each Find Information Response: past the budget.
    for _ in 0..2600 {
        db = db.desc(0x2904, &[0; 7]);
    }
    let mut run = Run::new(db.desc(0x2902, &[0, 0]));
    run.go();
    assert_eq!(run.batteries, [Battery { instance: 0, percent: 90 }]);
    assert!(run.peer.requests <= MAX_REQUESTS as usize);
    assert_eq!(run.peer.writes.len(), 1, "the level's CCCD was never reached");
    still_types(&mut run);
}

#[test]
fn a_battery_that_never_answers_times_the_bearer_out() {
    let mut peer = Peripheral::new(keyboard_mouse());
    peer.mute = Some(READ_REQ);
    let mut run = Run::with(peer);
    run.go();
    let kb = run.peer.db.handle_of(0x2a22);
    run.notify(kb, &KEY);
    let step = run.host.poll(run.now + TIMEOUT_MS);
    assert_eq!(step.outcome, Some(Outcome::Failed(HogpError::Timeout)));
    assert_eq!(step.input, [Input::KeyUp(0x04)]);
}
