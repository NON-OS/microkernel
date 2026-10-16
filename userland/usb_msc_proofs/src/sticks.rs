// NONOS Operating System (AGPL-3.0-or-later)
//! The sticks the tests plug in and bind: each has a serial number, as a
//! stick the driver serves again after a let-go must (scan/release.rs).

use crate::fixture::{bot_config, ready_unit};
use crate::server::serve::Server;
use crate::serving::{read, until_ready};
use crate::xhci::{setup, Answer, Port, PORT_FREE};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

/// A serial number string descriptor of `text`.
pub fn serial(text: &str) -> Vec<u8> {
    let mut s = vec![2 + 2 * text.len() as u8, 3];
    s.extend(text.bytes().flat_map(|b| [b, 0]));
    s
}

/// One device on root port 1 with a Bulk-Only interface, one unit and a
/// serial number, answering `answers`.
pub fn plug(answers: Vec<Answer>) {
    crate::xhci::script(answers);
    let (ports, serials) = (vec![Port { id: 1, owner: PORT_FREE }], vec![(1, serial("NONOS1"))]);
    setup(|d| (d.ports, d.config, d.serials) = (ports, bot_config(), serials));
}

/// A server with a stick of `blocks` bound on root port 1.
pub fn bound(blocks: u32) -> Server {
    plug(ready_unit(blocks));
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    server
}

/// The same, once the kernel has read a sector of it: from then on it
/// holds something of the medium (scan/check.rs).
pub fn held(blocks: u32) -> Server {
    let mut server = bound(blocks);
    setup(|d| d.answers = [Answer::Data(vec![0; 512]), OK].into_iter().collect());
    assert_eq!(read(&mut server), 0);
    server
}
