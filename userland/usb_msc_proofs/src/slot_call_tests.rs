// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's own xhci/call.rs and xhci/slot.rs, over a scripted IPC call
//! (xhci/ipc.rs): a call with no time left in its window is not sent and
//! fails E_TIMEDOUT; one the controller outlasts fails -110 in the kernel,
//! named E_TIMEDOUT, at the window's end or at the call's own 12 s; and a
//! slot is given back in a window of its own, whatever its caller had left.

use crate::clock::at;
use crate::protocol::{E_IO, E_TIMEDOUT};
use crate::xhci::{address_device, device, disable_slot, enable_slot, script, setup, Window};

#[test]
fn slot_requests_cross_the_wire_and_their_replies_are_checked() {
    script(vec![]);
    assert_eq!((enable_slot(1), address_device(1, 1, 4)), (Ok(1), Ok(())));
    disable_slot(1, 1);
    let asked = device(|d| (d.enabled, d.addressed.clone(), d.disabled.clone()));
    assert_eq!(asked, (1, vec![4], vec![1]));
    setup(|d| d.silent_ms = Some(100));
    assert_eq!(enable_slot(1), Err(E_IO), "the controller's own status");
}

#[test]
fn a_call_with_no_time_left_is_not_sent() {
    script(vec![]);
    let _spent = Window::open(0);
    assert_eq!((enable_slot(1), address_device(1, 1, 4)), (Err(E_TIMEDOUT), Err(E_TIMEDOUT)));
    assert_eq!(device(|d| (d.enabled, d.addressed.len())), (0, 0), "nothing was sent");
    disable_slot(1, 1);
    assert_eq!(device(|d| d.disabled.clone()), vec![1], "given back in a window of its own");
}

#[test]
fn a_call_the_controller_outlasts_fails_timed_out_at_its_bound() {
    script(vec![]);
    setup(|d| d.silent_ms = Some(600_000));
    let t0 = at();
    {
        let _step = Window::open(3_000);
        assert_eq!(address_device(1, 1, 4), Err(E_TIMEDOUT));
    }
    assert_eq!(at() - t0, 3_000, "at the window's end");
    let t0 = at();
    assert_eq!(enable_slot(1), Err(E_TIMEDOUT));
    assert_eq!(at() - t0, 12_000, "with none open, at the call's own timeout");
}
