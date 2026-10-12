// NONOS Operating System (AGPL-3.0-or-later)
//! Without entropy no device is addressed, and the address the device
//! carries is never taken in its place; once entropy comes, the search
//! goes on with the address drawn then.

use nonos_libc::{calls, lines, play, serial, set_entropy};

use crate::fake_xhci::answer;
use crate::scan::Scanner;
use crate::station::draw;
use crate::xhci::XhciBus;
use crate::{Bind, Found, Station};

fn take(_: XhciBus, _: &Found, _: Station) -> Bind<()> {
    Bind::Ours(())
}

#[test]
fn without_entropy_no_port_is_addressed_and_that_is_said_once() {
    let _one = serial();
    set_entropy(false);
    assert!(draw().is_none());
    play(|req| Some(answer(req)));
    let mut scanner = Scanner::new(b"t");
    for _ in 0..3 {
        assert!(scanner.pass(b"t", take).is_none());
    }
    assert!(calls().is_empty(), "no slot enabled, no device addressed or read");
    let said = String::from_utf8_lossy(&lines().concat()).into_owned();
    assert_eq!(said.matches("no entropy for a station address").count(), 1, "{said}");
    set_entropy(true);
    assert!(scanner.pass(b"t", take).is_some(), "bound once an address is drawn");
}
