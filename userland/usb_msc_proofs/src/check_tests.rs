// NONOS Operating System (AGPL-3.0-or-later)
//! A bound device that fails a request is asked whether it is still there
//! and what it holds: a stick pulled out is bound again only when it is back,
//! and a card swapped in a reader is served no more this boot. Another stick
//! was served in its place, and a swapped card at its new size.

use crate::fixture::{ready_unit, sense};
use crate::protocol::{E_AGAIN, E_IO, E_NODEV};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::serving::{idle, read, until_ready};
use crate::sticks::{bound, held, plug, serial};
use crate::xhci::{device, setup, Answer, Port, PORT_FREE};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

/// A READ the device fails at once, with `key` and `asc`, then `after`.
fn fails_read(key: u8, asc: u8, after: Vec<Answer>) {
    let failed = [Answer::Csw { residue: 512, status: 1 }, Answer::Data(sense(key, asc)), OK];
    setup(|d| d.answers = failed.into_iter().chain(after).collect());
}

#[test]
fn a_stick_pulled_out_is_let_go_and_only_it_is_bound_again() {
    let mut server = bound(1000);
    setup(|d| (d.ports.clear(), d.answers.clear()));
    assert_eq!(read(&mut server), E_AGAIN, "lost before anything was held: asked again");
    assert_eq!(server.scanner.report.stage, Stage::Gone, "checked in the step after the read");
    assert_eq!(device(|d| d.disabled.clone()), vec![1], "its slot given back");
    while !matches!(server.scanner.medium, Medium::Absent) {
        assert_eq!(read(&mut server), E_AGAIN);
        idle(&mut server);
    }
    assert_eq!(read(&mut server), E_NODEV);
    plug(ready_unit(2000));
    while device(|d| d.disabled.is_empty()) {
        idle(&mut server);
    }
    assert_eq!(server.scanner.report.stage, Stage::Other);
    assert_eq!(read(&mut server), E_NODEV, "another stick plugged in is not served");
    setup(|d| d.ports.clear());
    idle(&mut server);
    plug(ready_unit(1000));
    until_ready(&mut server, 2_000);
    assert!(matches!(server.scanner.medium, Medium::Ready(d) if d.blocks == 1000));
}

#[test]
fn a_card_swapped_in_a_reader_is_served_no_more() {
    let mut server = held(1000);
    fails_read(0x06, 0x28, vec![]);
    assert_eq!(read(&mut server), E_IO, "the read the swap failed");
    for _ in 0..1_000 {
        idle(&mut server);
        assert_eq!(read(&mut server), E_NODEV, "a card of the same size too");
    }
    assert_eq!(server.scanner.report.stage, Stage::Changed);
    let (enabled, disabled) = device(|d| (d.enabled, d.disabled.clone()));
    assert_eq!((enabled, disabled), (1, vec![1]), "the reader given back, not asked again");
}

#[test]
fn a_stick_plugged_back_into_another_port_is_served_there() {
    let mut server = bound(1000);
    setup(|d| (d.ports.clear(), d.answers.clear()));
    assert_eq!(read(&mut server), E_AGAIN, "lost before anything was held: asked again");
    idle(&mut server);
    plug(ready_unit(1000));
    let (ports, serials) = (vec![Port { id: 3, owner: PORT_FREE }], vec![(3, serial("NONOS1"))]);
    setup(|d| (d.ports, d.serials) = (ports, serials));
    until_ready(&mut server, 2_000);
    let port = |m: &Medium| if let Medium::Ready(d) = m { d.port } else { 0 };
    assert_eq!(port(&server.scanner.medium), 3, "the port is not identity");
}
