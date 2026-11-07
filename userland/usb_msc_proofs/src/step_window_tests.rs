// NONOS Operating System (AGPL-3.0-or-later)
//! How long a step may hold the server (scan/step_window.rs): short while a
//! device is served, so a block request queued behind it is answered in the
//! kernel's wait; longer while none is, so a call made as a crowd of capsules
//! holds the one CPU is answered in it, not given up and reported as a
//! server that did not answer (driver.xhci0, smp1, the desktop's spawn wave).

use crate::scan::{Medium, Scanner, SEARCH_STEP_MS, STEP_MS};
use crate::serving::{idle, STEP_BOUND_MS};
use crate::sticks::bound;

const CALL: &str = include_str!("../../capsule_driver_usb_msc/src/xhci/call.rs");
const SERVE: &str = include_str!("../../capsule_driver_usb_msc/src/server/serve.rs");
/// The kernel's wait for a reply (services/lifecycle/reply_wait.rs), the
/// margin kept inside it, and what a request answered at once takes.
const KERNEL_WAIT_MS: u64 = 35_000;
const MARGIN_MS: u64 = 2_000;

#[test]
fn a_step_that_only_lists_the_ports_has_the_long_window() {
    let mut scanner = Scanner::new();
    for m in [Medium::Scanning, Medium::Absent, Medium::Changed] {
        scanner.medium = m;
        assert_eq!(scanner.step_ms(), SEARCH_STEP_MS);
    }
}

#[test]
fn a_step_with_a_device_served_keeps_the_short_one() {
    let mut server = bound(1000);
    assert!(matches!(server.scanner.medium, Medium::Ready(_)));
    assert_eq!(server.scanner.step_ms(), STEP_MS);
    assert!(idle(&mut server) <= STEP_BOUND_MS);
}

#[test]
fn the_long_window_holds_a_whole_call_and_stays_inside_the_kernels_wait() {
    assert!(CALL.contains("const CALL_TIMEOUT_MS: u64 = 12_000;"));
    assert!(SEARCH_STEP_MS > 12_000, "one call's whole wait fits the step");
    assert!(STEP_BOUND_MS + MARGIN_MS <= KERNEL_WAIT_MS, "a request queued behind a search step");
    assert!(STEP_MS > 5_000 && STEP_MS < SEARCH_STEP_MS);
}

#[test]
fn the_server_opens_the_step_with_the_mediums_window() {
    assert!(SERVE.contains("Window::open(self.scanner.step_ms());"));
    assert!(!SERVE.contains("const STEP_MS"), "one window rule, in scan/step_window.rs");
}
