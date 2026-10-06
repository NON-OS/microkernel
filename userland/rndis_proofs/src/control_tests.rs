// NONOS Operating System (AGPL-3.0-or-later)
//! Binding QEMU usb-net's RNDIS configuration: every request in Linux
//! generic_rndis_bind's order and layout, and the control channel passing
//! over what is not its answer.

use std::time::Instant;

use nonos_usbnet::found::fetch;
use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::{Bind, Nic, Setup};

use crate::device::{indicate, set_cmplt, words, NIC_MAC, STATUS_MEDIA_CONNECT};
use crate::qemu::{honest, usb_net, Tamper};
use crate::rndis::bind;

const SEND: Setup = Setup::new(0x21, 0x00, 0, 0);
const GET: Setup = Setup::new(0xA1, 0x01, 0, 0);

pub fn bind_with(tamper: Tamper) -> (MockBus, Bind<crate::rndis::link::Rndis<MockBus>>) {
    let bus = usb_net(tamper);
    let found = fetch(&mut bus.clone(), 2).unwrap();
    bus.0.borrow_mut().calls.clear();
    let r = bind(bus.clone(), &found);
    (bus, r)
}

#[test]
fn qemu_usb_net_binds_in_configuration_two_in_linux_order() {
    let (bus, r) = bind_with(honest());
    let Bind::Ours(nic) = r else { panic!("not bound") };
    assert_eq!(nic.mac(), NIC_MAC);
    let calls = bus.0.borrow().calls.clone();
    assert!(matches!(calls[0], Call::Configure(p) if p.bulk_in == 0x82 && p.bulk_out == 0x02));
    assert_eq!(calls[1], Call::Out(Setup::set_configuration(2), vec![]));
    assert_eq!(calls[2], Call::Out(SEND, words(&[2, 24, 1, 1, 0, 4096])));
    assert_eq!(calls[3], Call::In(GET, 512));
    let mut query = words(&[4, 76, 2, 0x0101_0101, 48, 20, 0]);
    query.resize(76, 0);
    assert_eq!(calls[4], Call::Out(SEND, query));
    assert_eq!(calls[5], Call::In(GET, 512));
    assert_eq!(calls[6], Call::Out(SEND, words(&[5, 32, 3, 0x0001_010E, 4, 20, 0, 0x0D])));
    assert_eq!(calls[7], Call::In(GET, 512));
    assert_eq!(calls.len(), 8, "no SET_INTERFACE: the pipes are on alternate 0");
}

#[test]
fn indications_stale_answers_and_empty_polls_are_passed_over() {
    let tamper: Tamper = Box::new(|_, r| {
        let mut stale = r.clone();
        stale[8] ^= 0x40;
        vec![vec![0], indicate(STATUS_MEDIA_CONNECT), stale, set_cmplt(0, 0), r]
    });
    let (bus, r) = bind_with(tamper);
    assert!(matches!(r, Bind::Ours(_)));
    let polls = bus.0.borrow().calls.iter().filter(|c| matches!(c, Call::In(..))).count();
    assert_eq!(polls, 15, "five answers read for each of three commands");
}

#[test]
fn a_silent_device_times_out_on_the_clock_and_is_not_halted() {
    let start = Instant::now();
    let (bus, r) = bind_with(Box::new(|_, _| vec![]));
    assert!(matches!(r, Bind::Failed("RNDIS INITIALIZE failed", -110)));
    assert!(start.elapsed().as_millis() >= 1_000);
    assert!(nonos_libc::slept_ms() > 0, "it slept between polls instead of spinning");
    let halts = bus
        .0
        .borrow()
        .calls
        .iter()
        .filter(|c| matches!(c, Call::Out(s, m) if *s == SEND && m[0] == 3))
        .count();
    assert_eq!(halts, 0, "Linux halts only a device it initialized");
}
