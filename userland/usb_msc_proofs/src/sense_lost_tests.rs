// NONOS Operating System (AGPL-3.0-or-later)
//! A REQUEST SENSE the transport fails counts toward the three transport
//! failures in a row a device is given up after (scan/pace.rs), as any
//! other command does. It was taken as a unit not yet ready, and a device
//! that failed every sense was asked for its whole thirty seconds.

use crate::fixture::{sense, turs};
use crate::protocol::E_IO;
use crate::scan::report::Stage;
use crate::server::serve::Server;
use crate::serving::idle;
use crate::sticks::plug;
use crate::xhci::{device, Answer};

#[test]
fn a_sense_the_transport_fails_counts_toward_giving_the_device_up() {
    let ok = || Answer::Csw { residue: 0, status: 0 };
    // A TEST UNIT READY that fails, then a sense whose CSW is not one.
    let lost = || {
        [
            Answer::Csw { residue: 0, status: 1 },
            Answer::Data(sense(2, 4)),
            Answer::Data(vec![0; 13]),
        ]
    };
    let answers = [Answer::Data(vec![0; 36]), ok()].into_iter().chain((0..3).flat_map(|_| lost()));
    plug(answers.collect());
    let mut server = Server::new();
    while device(|d| d.disabled.is_empty()) {
        idle(&mut server);
    }
    let report = server.scanner.report;
    assert_eq!((report.stage, report.detail), (Stage::NotReady, E_IO));
    assert_eq!(turs(), 3, "given up on the third sense lost");
}
