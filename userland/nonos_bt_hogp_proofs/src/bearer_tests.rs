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

//! The bearer's rules: requests to this host refused, indications
//! confirmed, one transaction at a time (Vol 3, Part F, 3.3 and 3.4.1.1).

use crate::sim::{keyboard_mouse, Run};
use nonos_bt_hogp::host::{Host, Outcome, Step};
use nonos_bt_hogp::HogpError;

#[test]
fn a_request_to_this_host_is_refused_and_discovery_goes_on() {
    let (mut host, _) = Host::start(0);
    // Read Request for handle 3: Error Response, Request Not Supported.
    assert_eq!(host.on_pdu(&[0x0a, 0x03, 0x00], 1).send, [vec![0x01, 0x0a, 0, 0, 0x06]]);
    // A command is never answered.
    assert_eq!(host.on_pdu(&[0x52, 0x03, 0x00, 0x01], 2), Step::default());
    let mut run = Run::new(keyboard_mouse());
    run.deliver(&[0x02, 0x17, 0x00]);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
}

#[test]
fn indications_are_confirmed_and_reports_before_ready_ignored() {
    let db = keyboard_mouse();
    let kb = db.handle_of(0x2a22);
    let (mut host, _) = Host::start(0);
    let step =
        host.on_pdu(&[&[0x1d][..], &kb.to_le_bytes(), &[0, 0, 4, 0, 0, 0, 0, 0]].concat(), 1);
    assert_eq!((step.send, step.input), (vec![vec![0x1e]], vec![]));
    let mut run = Run::new(db);
    run.go();
    assert_eq!(run.notify(0x0099, &[0, 0, 4, 0, 0, 0, 0, 0]), []);
}

#[test]
fn responses_that_answer_nothing_end_discovery() {
    let fail = |pdu: &[u8]| {
        let (mut host, _) = Host::start(0);
        host.on_pdu(pdu, 1).outcome
    };
    let protocol = Some(Outcome::Failed(HogpError::Protocol));
    // A Write Response while a Find By Type Value is outstanding.
    assert_eq!(fail(&[0x13]), protocol);
    // An Error Response naming another request.
    assert_eq!(fail(&[0x01, 0x08, 0x01, 0x00, 0x0a]), protocol);
    assert_eq!(fail(&[]), protocol);
    let short = Some(Outcome::Failed(HogpError::Wire(nonos_bt_wire::WireError::Length)));
    assert_eq!(fail(&[0x07, 0x01, 0x00, 0x05]), short);
}
