// NONOS Operating System (AGPL-3.0-or-later)
//! The device served is the one first bound, for the whole boot. The kernel
//! chose its disk once (src/hardware/block_device/select.rs) and never looks
//! again, so after a let-go only the same unit, size and serial number is
//! served, and a stick let go without leaving its port is asked there
//! alone. The unfixed driver let the NONOS stick go on a power-on UNIT
//! ATTENTION and served the stick that answered first.

use crate::fixture::{bot_config, ready_unit, sense};
use crate::protocol::{E_AGAIN, E_IO};
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{idle, read, until_ready};
use crate::sticks::serial;
use crate::xhci::{device, script, setup, Answer, Port, PORT_FREE};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

pub(super) fn served(server: &Server) -> (u8, u64) {
    let Medium::Ready(d) = server.scanner.medium else { panic!("not ready") };
    (d.port, d.blocks)
}

/// The NONOS stick served on port 2; another stick of the same make and
/// size plugged into port 1 after, the two with `serials`; and the next read
/// failed with MEDIUM ERROR, then the check heard `check`.
pub(super) fn two_sticks(serials: Vec<(u8, Vec<u8>)>, check: Vec<Answer>) -> Server {
    script(ready_unit(1000));
    setup(|d| {
        (d.ports, d.config, d.serials) =
            (vec![Port { id: 2, owner: PORT_FREE }], bot_config(), serials)
    });
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    assert_eq!(served(&server), (2, 1000));
    setup(|d| d.ports.insert(0, Port { id: 1, owner: PORT_FREE }));
    let failed = [Answer::Csw { residue: 512, status: 1 }, Answer::Data(sense(0x03, 0x11)), OK];
    setup(|d| d.answers = failed.into_iter().chain(check).collect());
    assert_eq!(read(&mut server), E_IO);
    server
}

/// What the check hears: NOT READY, and the START UNIT it sends; then the
/// answers of a stick bound again.
pub(super) fn not_ready_then_bound() -> Vec<Answer> {
    let not_ready = [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(0x02, 0x04)), OK];
    not_ready.into_iter().chain([OK]).chain(ready_unit(1000)).chain(ready_unit(1000)).collect()
}

pub(super) fn nonos_and_other() -> Vec<(u8, Vec<u8>)> {
    vec![(1, serial("OTHER")), (2, serial("NONOS"))]
}

#[test]
fn a_power_on_said_once_is_asked_past_and_no_other_stick_is_asked() {
    let mut cap = 999u32.to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    let power_on = [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(0x06, 0x29)), OK];
    let check = power_on.into_iter().chain([OK, Answer::Data(cap), OK]).collect();
    let server = two_sticks(nonos_and_other(), check);
    assert_eq!(served(&server), (2, 1000), "the kernel's disk is still the NONOS stick");
    assert_eq!(device(|d| (d.addressed.clone(), d.answers.len())), (vec![2], 0));
}

#[test]
fn a_stick_plugged_in_beside_the_served_one_is_not_asked_after_a_let_go() {
    let mut server = two_sticks(nonos_and_other(), not_ready_then_bound());
    while !matches!(server.scanner.medium, Medium::Ready(_)) {
        assert_eq!(read(&mut server), E_AGAIN);
        idle(&mut server);
    }
    assert_eq!(served(&server), (2, 1000), "the NONOS stick, asked on its own port");
    assert_eq!(device(|d| d.addressed.clone()), vec![2, 2], "port 1's stick was never asked");
}
