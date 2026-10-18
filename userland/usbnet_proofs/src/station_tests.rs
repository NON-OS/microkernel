// NONOS Operating System (AGPL-3.0-or-later)
//! The station address every bind is handed: drawn from CryptoRandom, made
//! locally administered unicast, and drawn once a run, the same for every
//! bind in it; without entropy no device is addressed (`entropy`), and it
//! is never formatted (`format`).

mod entropy;
mod format;

use std::cell::RefCell;

use nonos_libc::{calls, play, serial};

use crate::fake_xhci::answer;
use crate::scan::NetScanner;
use crate::station::Station;
use crate::xhci::XhciBus;
use crate::{Bind, Bus, Found, Setup};

thread_local! {
    static HANDED: RefCell<Vec<[u8; 6]>> = const { RefCell::new(Vec::new()) };
}

fn keep(_: XhciBus, _: &Found, station: Station) -> Bind<()> {
    HANDED.with(|h| h.borrow_mut().push(station.bytes()));
    Bind::Failed("kept the address only", -5)
}

/// Binds by giving the device the address handed, as SET_NET_ADDRESS (0x82) does.
fn program(mut bus: XhciBus, _: &Found, station: Station) -> Bind<()> {
    match bus.control_out(Setup::class(false, 0x82, 0, 0), &station.bytes()) {
        Ok(()) => Bind::Ours(()),
        Err(e) => Bind::Failed("set address", e),
    }
}

#[test]
fn any_bytes_become_a_locally_administered_unicast_address() {
    let _one = serial();
    let factory = [0x00, 0x0e, 0xc6, 0x87, 0x72, 0x01];
    for raw in [factory, [0xff; 6], [0; 6], [0x01, 0, 0x5e, 0, 0, 1]] {
        let mac = Station::from_random(raw).bytes();
        assert_eq!(mac[0] & 0x03, 0x02, "{raw:02x?}");
        assert_eq!(mac[1..], raw[1..], "only the two meaningful bits move");
    }
}

#[test]
fn every_bind_of_a_run_is_handed_the_one_address_drawn_for_it() {
    let _one = serial();
    play(|req| Some(answer(req)));
    let mut scanner = NetScanner::new(b"t");
    scanner.pass(b"t", keep);
    scanner.pass(b"t", keep);
    let handed = HANDED.with(|h| h.borrow().clone());
    assert_eq!(handed.len(), 2);
    assert_eq!(handed[0], handed[1]);
    assert_eq!(handed[0][0] & 0x03, 0x02);
}

#[test]
fn a_rebind_after_a_loss_programs_the_address_the_stack_already_has() {
    let _one = serial();
    play(|req| Some(answer(req)));
    let mut scanner = NetScanner::new(b"t");
    let first = scanner.pass(b"t", program).expect("bound");
    scanner.lost(first.xhci, first.port);
    assert!(scanner.pass(b"t", program).is_some(), "bound again on the same port");
    // The data stage of each SET_NET_ADDRESS: the address programmed.
    let asked = calls();
    let set: Vec<&[u8]> =
        asked.iter().filter(|a| a.req.get(23) == Some(&0x82)).map(|a| &a.req[30..]).collect();
    assert_eq!(set.len(), 2);
    assert_eq!(set[0], set[1], "net.core asks for the address once a port and keeps it");
}
