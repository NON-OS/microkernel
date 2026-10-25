// NONOS Operating System (AGPL-3.0-or-later)
//! Without entropy no device is addressed, and the address the device
//! carries is never taken in its place; once entropy comes, the search
//! goes on with the address drawn then. A class driver that is not a
//! network one (audio) searches with no address and needs none.

use nonos_libc::{calls, lines, play, serial, set_entropy};

use crate::fake_xhci::answer;
use crate::scan::{NetScanner, Scanner};
use crate::station::draw;
use crate::xhci::XhciBus;
use crate::{Bind, Found, Station};

fn take(_: XhciBus, _: &Found, _: Station) -> Bind<()> {
    Bind::Ours(())
}

fn take_class(_: XhciBus, _: &Found) -> Bind<()> {
    Bind::Ours(())
}

#[test]
fn without_entropy_no_port_is_addressed_and_that_is_said_once() {
    let _one = serial();
    set_entropy(false);
    assert!(draw().is_none());
    play(|req| Some(answer(req)));
    let mut scanner = NetScanner::new(b"t");
    for _ in 0..3 {
        assert!(scanner.pass(b"t", take).is_none());
    }
    assert!(calls().is_empty(), "no slot enabled, no device addressed or read");
    let said = String::from_utf8_lossy(&lines().concat()).into_owned();
    assert_eq!(said.matches("no entropy for a station address").count(), 1, "{said}");
    set_entropy(true);
    assert!(scanner.pass(b"t", take).is_some(), "bound once an address is drawn");
}

#[test]
fn a_class_drivers_search_binds_without_entropy() {
    let _one = serial();
    set_entropy(false);
    play(|req| Some(answer(req)));
    let before = lines().len();
    assert!(Scanner::new().pass(b"t", take_class).is_some(), "bound, no address drawn");
    let said = String::from_utf8_lossy(&lines()[before..].concat()).into_owned();
    assert!(!said.contains("no entropy"), "{said}");
    set_entropy(true);
}
