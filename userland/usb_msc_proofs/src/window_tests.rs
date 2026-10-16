// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's call window (xhci/window.rs) as it ships, on the test clock:
//! a call waits at most what the open window has left and never past its
//! own timeout, a window opened inside another has time of its own and
//! gives the outer one back when it closes, and with none open a call
//! waits its own timeout. The scripted controller keeps a copy per test
//! thread; this is the one the capsule runs.

#[path = "../../capsule_driver_usb_msc/src/xhci/window.rs"]
mod window;

use crate::clock::advance;
use window::{left_ms, Window};

/// One test: the window is a static of the driver's, and tests run side by
/// side.
#[test]
fn a_window_bounds_each_call_and_one_inside_it_has_time_of_its_own() {
    assert_eq!(left_ms(12_000), 12_000, "none open: the call's own timeout");
    let step = Window::open(6_000);
    assert_eq!((left_ms(12_000), left_ms(100)), (6_000, 100));
    advance(5_000);
    assert_eq!(left_ms(12_000), 1_000);
    {
        let _reset = Window::open(8_000);
        assert_eq!(left_ms(12_000), 8_000, "a reset after a timeout has time of its own");
        advance(8_000);
        assert_eq!(left_ms(12_000), 0, "and is spent in turn");
    }
    assert_eq!(left_ms(12_000), 0, "the step's window is back, and spent");
    drop(step);
    assert_eq!(left_ms(12_000), 12_000);
    let _long = Window::open(u64::MAX);
    assert!(left_ms(u64::MAX) < u64::MAX, "no window reads as none open");
}
