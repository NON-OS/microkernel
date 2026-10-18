// NONOS Operating System (AGPL-3.0-or-later)
//! The search, run whole against a played driver.xhci0: what a slow
//! device and a failing one cost the thread that answers net.core.

use nonos_libc::{advance, calls, lines, play, serial};

use crate::fake_xhci::{answer, op, OP_CONTROL_TRANSFER, OP_DISABLE_SLOT, OP_ENABLE_SLOT};
use crate::scan::Scanner;
use crate::xhci::XhciBus;
use crate::{Bind, Bus, Found, Setup, Station};

fn count(want: u16) -> usize {
    calls().iter().filter(|a| op(&a.req) == want).count()
}

/// A binding that keeps the device talking for as long as it answers.
fn chatty(mut bus: XhciBus, _: &Found, _: Station) -> Bind<()> {
    for _ in 0..100 {
        if let Err(e) = bus.control_in(Setup::get_descriptor(1, 0), &mut [0u8; 18]) {
            return Bind::Failed("device read", e);
        }
    }
    Bind::Failed("read 100 times", -5)
}

fn refuse(_: XhciBus, _: &Found, _: Station) -> Bind<()> {
    Bind::Failed("refused", -5)
}

fn take(_: XhciBus, _: &Found, _: Station) -> Bind<()> {
    Bind::Ours(())
}

#[test]
fn a_bind_that_keeps_a_slow_device_talking_is_cut_off_by_its_budget() {
    let _one = serial();
    play(|req| {
        advance(1_000);
        Some(answer(req))
    });
    assert!(Scanner::new(b"t").pass(b"t", chatty).is_none());
    assert!(count(OP_CONTROL_TRANSFER) < 30, "{} transfers", count(OP_CONTROL_TRANSFER));
    let last = calls().last().map(|a| op(&a.req));
    assert_eq!(last, Some(OP_DISABLE_SLOT), "the slot is given back after the budget");
}

#[test]
fn a_port_that_keeps_failing_is_tried_less_and_less_often() {
    let _one = serial();
    play(|req| Some(answer(req)));
    let mut scanner = Scanner::new(b"t");
    for _ in 0..4 * 3600 {
        advance(1_000);
        assert!(scanner.pass(b"t", refuse).is_none());
    }
    assert!(count(OP_ENABLE_SLOT) <= 60, "{} binds in four hours", count(OP_ENABLE_SLOT));
}

#[test]
fn the_device_model_is_named_only_when_a_bind_fails() {
    let _one = serial();
    play(|req| Some(answer(req)));
    assert!(Scanner::new(b"t").pass(b"t", take).is_some());
    let said = lines().concat();
    assert!(!said.windows(9).any(|w| w == b"0525:a4a2"), "{}", String::from_utf8_lossy(&said));
    play(|req| Some(answer(req)));
    assert!(Scanner::new(b"t").pass(b"t", refuse).is_none());
    assert!(lines().concat().windows(9).any(|w| w == b"0525:a4a2"));
}
