// NONOS Operating System (AGPL-3.0-or-later)
//! A stick that binds, then fails every request and its check, is let go
//! each time, and its port keeps the count: it is bound again at once a few
//! times, then after a wait that doubles. Each let-go set its port as new,
//! and the stick was reset and bound again without end.

use crate::clock::at;
use crate::fixture::{ready_unit, sense};
use crate::protocol::E_IO;
use crate::scan::Medium;
use crate::serving::{idle, read};
use crate::sticks::bound;
use crate::xhci::{device, setup, Answer};

#[test]
fn a_stick_that_fails_every_request_backs_off() {
    let ok = || Answer::Csw { residue: 0, status: 0 };
    let check = || Answer::Csw { residue: 0, status: 1 };
    let mut server = bound(1000);
    let mut binds = vec![];
    // Bounded, so a stick that is not bound again fails the test, not hangs it.
    for _ in 0..2_000 {
        if binds.len() == 5 {
            break;
        }
        if matches!(server.scanner.medium, Medium::Ready(_)) {
            binds.push(at());
            let failed = [check(), Answer::Data(sense(0x03, 0x11)), ok()];
            let not_ready = [check(), Answer::Data(sense(0x02, 0x04)), ok(), ok()];
            let next = failed.into_iter().chain(not_ready).chain(ready_unit(1000));
            setup(|d| d.answers = next.collect());
            assert_eq!(read(&mut server), E_IO);
        }
        idle(&mut server);
    }
    assert_eq!(binds.len(), 5, "bound five times: {:?}", server.scanner.report);
    let gaps: Vec<u64> = binds.windows(2).map(|w| w[1] - w[0]).collect();
    assert!(gaps[..2].iter().all(|&g| g < 1_000), "bound again at once at first: {gaps:?}");
    assert!(gaps[2] >= 5_000 && gaps[3] >= 10_000, "then after a wait that doubles: {gaps:?}");
    assert_eq!(device(|d| d.enabled), 5);
}
