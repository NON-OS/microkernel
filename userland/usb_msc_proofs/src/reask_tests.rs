// NONOS Operating System (AGPL-3.0-or-later)
//! A medium change said while a device is asked again after a let-go: said
//! by the unit served, its card may be another of the same size, and nothing
//! tells them apart, so serving ends, even when that asking fails and a
//! later one hears no change; said by another device, it is that device's.
//! The unfixed driver forgot the change and served the stick again.

use super::identity_tests::served;
use crate::fixture::{bot_config, ready_unit, sense};
use crate::protocol::{E_AGAIN, E_IO, E_NODEV};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{idle, read, until_ready};
use crate::sticks::{bound, serial};
use crate::xhci::{device, script, setup, Answer, Port, PORT_FREE};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

fn fails(key: u8, asc: u8) -> [Answer; 3] {
    [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, asc)), OK]
}

/// A READ that fails, a check that hears NOT READY and lets the stick go,
/// then the stick asked again: INQUIRY, and a medium change said, then `after`.
fn let_go_then_changed(after: Vec<Answer>) -> Server {
    let mut server = bound(1000);
    let check = fails(0x02, 0x04).into_iter().chain([OK]);
    let again = [Answer::Data(vec![0; 36]), OK].into_iter().chain(fails(0x06, 0x28));
    let answers = fails(0x03, 0x11).into_iter().chain(check).chain(again).chain(after);
    setup(|d| d.answers = answers.collect());
    assert_eq!(read(&mut server), E_IO);
    while matches!(server.scanner.medium, Medium::Scanning) {
        idle(&mut server);
    }
    server
}

#[test]
fn the_served_stick_saying_its_medium_changed_when_asked_again_is_served_no_more() {
    let mut cap = 999u32.to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    let mut server = let_go_then_changed(vec![OK, Answer::Data(cap), OK]);
    assert_eq!(server.scanner.report.stage, Stage::Changed);
    assert_eq!(read(&mut server), E_NODEV);
}

#[test]
fn a_change_said_in_an_asking_that_failed_is_not_forgotten_by_the_next() {
    let lost = || Answer::Data(vec![0; 13]);
    let after = [lost(), lost(), lost()].into_iter().chain(ready_unit(1000)).collect();
    let mut server = let_go_then_changed(after);
    assert_eq!(server.scanner.report.stage, Stage::Changed);
    assert_eq!(read(&mut server), E_NODEV);
}

#[test]
fn a_medium_change_another_device_said_is_its_own() {
    script(ready_unit(1000));
    let serials = vec![(1, serial("OTHER")), (3, serial("NONOS"))];
    let port = vec![Port { id: 3, owner: PORT_FREE }];
    setup(|d| (d.ports, d.config, d.serials) = (port, bot_config(), serials));
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    setup(|d| (d.ports.clear(), d.answers.clear()));
    assert_eq!(read(&mut server), E_AGAIN, "the NONOS stick pulled out: asked again");
    let reader = [Answer::Data(vec![0; 36]), OK].into_iter().chain(fails(0x06, 0x28));
    let empty = fails(0x02, 0x3A).into_iter().chain(fails(0x02, 0x3A));
    let answers = reader.chain(empty).chain(ready_unit(1000)).collect();
    let ports = vec![Port { id: 1, owner: PORT_FREE }, Port { id: 3, owner: PORT_FREE }];
    setup(|d| (d.ports, d.answers) = (ports, answers));
    until_ready(&mut server, 20_000);
    assert_eq!(served(&server), (3, 1000), "an empty reader's change is not the NONOS stick's");
    assert_eq!(device(|d| d.addressed.clone()), vec![3, 1, 3]);
}
