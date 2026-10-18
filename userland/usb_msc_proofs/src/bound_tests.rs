// NONOS Operating System (AGPL-3.0-or-later)
//! Every wait of a probe ends by the clock with a name, and no request
//! waits longer than one step: a unit that never comes ready, and a device
//! that stops answering mid-probe.

use crate::clock::at;
use crate::fixture::{one_sector, sense};
use crate::protocol::{E_AGAIN, E_NODEV, E_TIMEDOUT, OP_BLK_READ};
use crate::scan::report::Stage;
use crate::server::serve::Server;
use crate::serving::{ask, idle};
use crate::sticks::plug;
use crate::xhci::{device, setup, Answer};

/// A controller that never answers: each call waits out its window.
const HUNG_MS: u64 = 600_000;
const START_UNIT: u8 = 0x1B;

fn answered_at_once(server: &mut Server) {
    let t0 = at();
    let reply = ask(server, 0, OP_BLK_READ, &one_sector(7));
    assert!(reply.status == E_AGAIN || reply.status == E_NODEV, "answered {}", reply.status);
    assert_eq!(reply.at, t0, "no device was asked");
}

#[test]
fn a_unit_that_never_comes_ready_is_given_up_by_the_clock() {
    let ok = || Answer::Csw { residue: 0, status: 0 };
    let not_ready =
        || [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(0x02, 0x04)), ok()];
    let mut answers = vec![Answer::Data(vec![0; 36]), ok()];
    for round in 0..1_000 {
        answers.extend(not_ready());
        answers.extend((round == 0).then(ok));
    }
    plug(answers);
    let (mut server, start) = (Server::new(), at());
    while device(|d| d.disabled.is_empty()) {
        answered_at_once(&mut server);
        assert!(idle(&mut server) <= 20);
    }
    assert!(at() - start <= 30_000 + 200, "given up at {} ms", at() - start);
    let report = server.scanner.report;
    assert_eq!((report.stage, report.detail), (Stage::NotReady, E_TIMEDOUT));
    let starts = device(|d| d.commands.iter().filter(|&&op| op == START_UNIT).count());
    assert_eq!(starts, 1, "one START UNIT");
}

#[test]
fn a_device_that_stops_answering_mid_probe_holds_no_step_past_its_bound() {
    plug(vec![]);
    let mut server = Server::new();
    idle(&mut server);
    assert_eq!(device(|d| d.enabled), 1, "bound, and its units to be asked");
    setup(|d| d.silent_ms = Some(HUNG_MS));
    while device(|d| d.disabled.is_empty()) {
        answered_at_once(&mut server);
        idle(&mut server);
    }
    assert_eq!(server.scanner.report.detail, E_TIMEDOUT);
}
