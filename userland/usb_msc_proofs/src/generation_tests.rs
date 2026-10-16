// NONOS Operating System (AGPL-3.0-or-later)
//! The state reply (OP_GET_STATE) ends with a re-bind generation, raised
//! each time a device is served after the first, so the kernel can read
//! what it trusts of the disk again before it writes after one. A re-bind
//! left nothing on the wire, and the kernel wrote on as before.

use crate::fixture::{frame, ready_unit, sense};
use crate::protocol::{E_AGAIN, E_IO, HDR_LEN, OP_GET_STATE, STATUS_LEN};
use crate::scan::report::REPORT_LEN;
use crate::server::respond::sent;
use crate::server::serve::Server;
use crate::serving::{read, until_ready};
use crate::sticks::bound;
use crate::xhci::{setup, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };
/// The counters and the search report, which the generation follows.
const BEFORE: usize = 48 + REPORT_LEN;

fn fails(key: u8, asc: u8) -> [Answer; 3] {
    [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, asc)), OK]
}

/// The state reply's length and the generation at its end.
fn generation(server: &mut Server) -> (usize, u32) {
    let mut tx = vec![0u8; 64 * 1024];
    let _ = sent();
    server.serve(0, &frame(OP_GET_STATE, 9, &[]), &mut tx);
    let at = HDR_LEN + STATUS_LEN + BEFORE;
    (sent()[0].data, u32::from_le_bytes([tx[at], tx[at + 1], tx[at + 2], tx[at + 3]]))
}

#[test]
fn the_state_reply_ends_with_a_generation_raised_on_every_bind_after_the_first() {
    let mut server = bound(1000);
    assert_eq!(generation(&mut server), (BEFORE + 4, 0), "the first bind");
    for rebinds in 1..=2 {
        let check = fails(0x02, 0x04).into_iter().chain([OK]).chain(ready_unit(1000));
        setup(|d| d.answers = fails(0x03, 0x11).into_iter().chain(check).collect());
        assert_eq!(read(&mut server), E_IO);
        until_ready(&mut server, 2_000);
        assert_eq!(generation(&mut server), (BEFORE + 4, rebinds), "let go and bound again");
    }
    let mut cap = 1999u32.to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    let afresh = fails(0x06, 0x28).into_iter().chain([OK, Answer::Data(cap), OK]);
    setup(|d| d.answers = afresh.collect());
    assert_eq!(read(&mut server), E_AGAIN);
    assert_eq!(generation(&mut server), (BEFORE + 4, 3), "a medium served afresh");
}
