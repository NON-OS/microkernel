// NONOS Operating System (AGPL-3.0-or-later)
//! A stick that times out every READ yet passes every check was kept for
//! the boot: each request cost the kernel 18 s and a refusal, and no disk
//! was ever chosen. Three requests failed in a row now let it go
//! (scan/keep.rs). One with a serial number is searched for again; once its
//! port waits, the kernel is told there is no device until the wait ends,
//! and then it is bound again and times out again. One without ends serving.

use crate::answers::passes_check;
use crate::clock::at;
use crate::fixture::{one_sector, ready_unit};
use crate::kernel::generation;
use crate::protocol::{E_AGAIN, E_NODEV, E_TIMEDOUT, OP_BLK_READ};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{answer, idle, read, until_ready};
use crate::sticks::{bound, plug};
use crate::xhci::setup;

/// The kernel reads from a stick silent under every READ that passes every
/// check and bind, until it is told there is no device; the search runs to
/// its end between reads. The statuses.
fn dead_stick(server: &mut Server) -> Vec<i32> {
    let mut said = vec![];
    while said.last() != Some(&E_NODEV) && said.len() < 20 {
        if matches!(server.scanner.medium, Medium::Ready(_)) {
            setup(|d| d.silent_ms = Some(600_000));
            said.push(answer(server, 0, OP_BLK_READ, &one_sector(7)).status);
            setup(|d| (d.silent_ms, d.answers) = (None, passes_check(1000).into()));
            idle(server);
            continue;
        }
        setup(|d| d.answers = ready_unit(1000).into());
        said.push(read(server));
        for _ in 0..10_000 {
            if !matches!(server.scanner.medium, Medium::Scanning) {
                break;
            }
            idle(server);
        }
    }
    said
}

#[test]
fn a_stick_timing_out_every_read_is_let_go_after_three_and_bound_again_after_each_port_wait() {
    let mut server = bound(1000);
    let t0 = at();
    let (t, again) = (E_TIMEDOUT, E_AGAIN);
    let said = dead_stick(&mut server);
    assert_eq!(said[..4], [t, t, t, again], "searched for again after three");
    // Bound again at once twice, and let go at its next failure: it has
    // moved nothing since. Then its port waits, and E_NODEV is answered.
    assert_eq!(said[4..], [t, again, t, again, E_NODEV]);
    assert!(at() - t0 < 180_000, "no device after {} ms", at() - t0);
    assert_eq!(generation(&mut server), 4, "two lost phases served afresh, and two binds");
    // Not given up for good: bound again, and told, once its port's wait ends.
    let t1 = at();
    setup(|d| d.answers = ready_unit(1000).into());
    until_ready(&mut server, 2_000);
    assert!(at() - t1 >= 5_000, "bound again before its port's wait ended");
    assert_eq!(generation(&mut server), 5);
    assert_eq!(dead_stick(&mut server), [t, again, E_NODEV], "let go again at once");
}

#[test]
fn one_with_no_serial_number_is_served_no_more_after_three() {
    plug(ready_unit(1000));
    setup(|d| d.serials.clear());
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    assert_eq!(dead_stick(&mut server), [E_TIMEDOUT, E_TIMEDOUT, E_TIMEDOUT, E_NODEV]);
    assert_eq!(server.scanner.report.stage, Stage::Changed);
}
