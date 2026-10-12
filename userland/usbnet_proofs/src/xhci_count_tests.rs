// NONOS Operating System (AGPL-3.0-or-later)
//! Counts and lengths in driver.xhci0's replies that reach past the bytes
//! that came, or past what was asked for: each is refused, none is read.

use nonos_libc::{play, serial};

use crate::fake_xhci::{reply, SLOT};
use crate::scan::Port;
use crate::xhci::{address_device, connected_ports, enable_slot, E_IO};
use crate::xhci_client_tests::bus;
use crate::{Bus, Setup};

fn with(data: &'static [u8]) {
    play(move |req| Some(reply(req, 0, data)));
}

#[test]
fn counts_and_lengths_past_what_came_are_refused() {
    let _one = serial();
    with(&[2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(connected_ports(0x4011), Err(E_IO), "two ports, one sent");
    with(&[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
    let one = Port { id: 1, owner: 0, changed: false };
    assert_eq!(connected_ports(0x4011), Ok(vec![one]));
    with(&[19, 0]);
    let mut dev = [0u8; 18];
    assert_eq!(bus().control_in(Setup::get_descriptor(1, 0), &mut dev), Err(E_IO));
    with(&[0, 0, 0, 0]);
    assert_eq!(enable_slot(0x4011), Err(E_IO), "slot 0 is no slot");
    with(&[SLOT, 9, 0, 0, 0, 0, 0, 0]);
    assert_eq!(address_device(0x4011, SLOT, 1), Err(E_IO), "another port addressed");
    with(&[8, 0, 0, 0, 1, 2]);
    assert_eq!(bus().bulk_in_poll(&mut [0u8; 64]), Err(E_IO), "8 moved, 2 came");
    with(&[65, 0, 0, 0]);
    assert_eq!(bus().bulk_in_poll(&mut [0u8; 64]), Err(E_IO), "more than was armed");
    play(|req| Some(reply(req, -11, &[])));
    assert_eq!(bus().bulk_in_poll(&mut [0u8; 64]), Ok(None), "nothing yet");
}
