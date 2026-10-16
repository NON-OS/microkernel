// NONOS Operating System (AGPL-3.0-or-later)
//! Which stick is served again after a let-go: only one the driver can tell
//! is the stick it served. Two sticks of one make with no serial number, or
//! with one serial number, are common, and the stick plugged in beside the
//! NONOS stick was served in its place. A stick with no serial number is
//! served no more after a let-go; one let go without leaving its port is
//! asked on that port alone; one pulled out and plugged in again is told
//! from another by its serial number, on whatever port.

use super::identity_tests::{nonos_and_other, not_ready_then_bound, served, two_sticks};
use crate::fixture::{bot_config, ready_unit};
use crate::protocol::{E_AGAIN, E_NODEV};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{idle, read, until_ready};
use crate::sticks::serial;
use crate::xhci::{device, script, setup, Port, PORT_FREE};

#[test]
fn sticks_with_no_serial_number_are_served_no_more_after_a_let_go() {
    let mut server = two_sticks(vec![], not_ready_then_bound());
    for _ in 0..1_000 {
        idle(&mut server);
    }
    assert!(matches!(server.scanner.medium, Medium::Changed), "{:?}", server.scanner.report);
    assert_eq!(read(&mut server), E_NODEV);
    assert_eq!(device(|d| d.addressed.clone()), vec![2], "neither stick was asked again");
}

#[test]
fn a_stick_beside_the_served_one_with_its_serial_number_is_not_taken() {
    let one = vec![(1, serial("0123456789AB")), (2, serial("0123456789AB"))];
    let mut server = two_sticks(one, not_ready_then_bound());
    until_ready(&mut server, 2_000);
    assert_eq!(served(&server), (2, 1000), "the NONOS stick, on its own port");
    assert_eq!(device(|d| d.addressed.clone()), vec![2, 2]);
}

#[test]
fn a_stick_pulled_out_and_plugged_in_again_is_told_from_another_by_its_serial() {
    script(ready_unit(1000));
    let port = vec![Port { id: 2, owner: PORT_FREE }];
    setup(|d| (d.ports, d.config, d.serials) = (port, bot_config(), nonos_and_other()));
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    setup(|d| (d.ports.clear(), d.answers.clear()));
    assert_eq!(read(&mut server), E_AGAIN, "lost before anything was held: asked again");
    assert_eq!(server.scanner.report.stage, Stage::Gone);
    let ports = vec![Port { id: 1, owner: PORT_FREE }, Port { id: 3, owner: PORT_FREE }];
    let serials = vec![(1, serial("OTHER")), (3, serial("NONOS"))];
    let answers = ready_unit(1000).into_iter().chain(ready_unit(1000)).collect();
    setup(|d| (d.ports, d.serials, d.answers) = (ports, serials, answers));
    until_ready(&mut server, 2_000);
    assert_eq!(served(&server), (3, 1000), "the NONOS stick, plugged into port 3");
    assert_eq!(device(|d| d.addressed.clone()), vec![2, 1, 3], "port 1's stick asked, not taken");
}
