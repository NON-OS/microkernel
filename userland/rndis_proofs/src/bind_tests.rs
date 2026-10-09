// NONOS Operating System (AGPL-3.0-or-later)
//! Each way a bind stops has its name and errno, and a device stopped
//! after INITIALIZE gets a HALT, as Linux generic_rndis_bind leaves it.

use nonos_usbnet::found::fetch;
use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::Bind;

use crate::control_tests::bind_with;
use crate::qemu::{honest, usb_net, Tamper};
use crate::rndis::bind;

/// QEMU's completion of request type `kind` with word `at` set to `v`.
fn word(kind: u32, at: usize, v: u32) -> Tamper {
    Box::new(move |k, mut r| {
        if k == kind {
            r[at..at + 4].copy_from_slice(&v.to_le_bytes());
        }
        vec![r]
    })
}

pub fn failed(tamper: Tamper) -> (&'static str, i32, bool) {
    let (bus, r) = bind_with(tamper);
    let Bind::Failed(what, e) = r else { panic!("bound") };
    (what, e, halted(&bus))
}

fn halted(bus: &MockBus) -> bool {
    let last = bus.0.borrow().calls.last().cloned();
    last == Some(Call::Out(
        nonos_usbnet::Setup::new(0x21, 0, 0, 0),
        crate::device::words(&[3, 12, 0]),
    ))
}

#[test]
fn a_refused_initialize_is_named_and_not_halted() {
    assert_eq!(failed(word(2, 12, 0xC000_0001)), ("RNDIS INITIALIZE failed", -47, false));
}

#[test]
fn a_medium_other_than_802_3_or_a_small_transfer_size_halts_the_device() {
    assert_eq!(failed(word(2, 28, 1)), ("medium not 802.3", -5, true));
    assert_eq!(failed(word(2, 36, 1558)), ("device transfer size under one frame", -5, true));
}

#[test]
fn a_refused_packet_filter_halts_the_device() {
    assert_eq!(failed(word(5, 12, 0xC000_0001)), ("RNDIS packet filter refused", -47, true));
}

#[test]
fn a_refused_configuration_is_named_and_an_ecm_only_device_is_not_ours() {
    let found = fetch(&mut usb_net(honest()), 2).unwrap();
    let bus = usb_net(honest());
    bus.0.borrow_mut().respond = Box::new(|c| match c {
        Call::Out(s, _) if s.request == 0x09 => Err(-32),
        _ => Ok(vec![]),
    });
    assert!(matches!(bind(bus, &found), Bind::Failed("SET_CONFIGURATION refused", -32)));
    let mut ecm_only = fetch(&mut usb_net(honest()), 2).unwrap();
    ecm_only.configs.remove(0);
    assert!(matches!(bind(usb_net(honest()), &ecm_only), Bind::NotOurs));
}
