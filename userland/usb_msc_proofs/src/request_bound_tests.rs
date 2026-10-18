// NONOS Operating System (AGPL-3.0-or-later)
//! A block request on a device that stops answering is answered within its
//! own window and the reset after it, inside the kernel's 35 s wait for the
//! reply; the unfixed driver held one 72 s of the controller's time. The
//! device is then checked before it serves another, and a request the
//! kernel sends while the check runs is answered inside that wait too.

use crate::clock::at;
use crate::fixture::{one_sector, ready_unit};
use crate::protocol::{E_AGAIN, E_TIMEDOUT, OP_BLK_READ};
use crate::scan::Medium;
use crate::serving::{answer, ask, idle, read};
use crate::sticks::bound;
use crate::xhci::{device, setup};

/// server/serve.rs REQUEST_MS, and disk/recover.rs RECOVERY_MS.
const REQUEST_BOUND_MS: u64 = 18_000 + 8_000;
/// The kernel's wait for a reply (services/lifecycle/reply_wait.rs), and
/// the margin kept inside it.
const KERNEL_WAIT_MS: u64 = 35_000;
const MARGIN_MS: u64 = 2_000;

#[test]
fn a_block_request_is_answered_within_its_own_window() {
    let mut server = bound(1000);
    setup(|d| d.silent_ms = Some(600_000));
    let t0 = at();
    let reply = ask(&mut server, 0, OP_BLK_READ, &one_sector(7));
    assert_eq!(reply.status, E_TIMEDOUT);
    assert!(reply.at - t0 <= REQUEST_BOUND_MS, "answered after {} ms", reply.at - t0);
    assert!(matches!(server.scanner.medium, Medium::Scanning), "checked in the step after: let go");
    assert_eq!(device(|d| d.disabled.len()), 1);
    assert_eq!(read(&mut server), E_AGAIN, "checked before it serves another");
}

/// A request queued as the check starts waits for the check, then for its
/// own window and reset. The worst is a device that passes the check as
/// slowly as the step lets it, then falls silent: each call it answers in
/// the check costs `call_ms`.
#[test]
fn a_request_queued_during_the_check_is_answered_within_the_kernels_wait() {
    for call_ms in [1, 250, 500, 999, 1_000, 1_001, 1_200, 2_000, 6_000, 7_000] {
        let mut server = bound(1000);
        setup(|d| d.silent_ms = Some(600_000));
        assert_eq!(answer(&mut server, 0, OP_BLK_READ, &one_sector(7)).status, E_TIMEDOUT);
        let cap = ready_unit(1000).split_off(2);
        setup(|d| (d.silent_ms, d.call_ms, d.answers) = (None, call_ms, cap.into()));
        let sent = at();
        idle(&mut server);
        let kept = matches!(server.scanner.medium, Medium::Ready(_));
        assert_eq!(kept, call_ms <= 1_000, "kept when its six calls fit the step");
        setup(|d| d.silent_ms = Some(600_000));
        let waited = ask(&mut server, 0, OP_BLK_READ, &one_sector(7)).at - sent;
        assert!(waited + MARGIN_MS <= KERNEL_WAIT_MS, "{call_ms} ms a call: {waited} ms");
    }
}
