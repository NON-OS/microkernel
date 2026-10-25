// NONOS Operating System (AGPL-3.0-or-later)
//! The station address: drawn for the run, never the iMACAddress the
//! device carries. A device that takes SET_NET_ADDRESS is given it, with
//! its data interface in alternate setting 0 as NCM 1.0, 6.2.2 asks; any
//! other has its packet filter opened to every frame instead.

use nonos_usbnet::mock::{drawn, Call, MockBus};
use nonos_usbnet::{Bind, Nic, Setup};

use crate::device::{calls, Dev};
use crate::ncm::bind;
use crate::spec::MAC;

/// bmNetworkCapabilities D1: the device takes SetNetAddress.
const CAP_NET_ADDRESS: u8 = 1 << 1;

fn filter_of(c: &[Call]) -> Option<u16> {
    c.iter().find_map(|x| match x {
        Call::Out(s, _) if s.request == 0x43 => Some(s.value),
        _ => None,
    })
}

fn bound(dev: &Dev) -> ([u8; 6], MockBus) {
    let (found, bus) = (dev.found(), dev.bus());
    let Bind::Ours(nic) = bind(bus.clone(), &found, drawn()) else { panic!("not bound") };
    let mac = nic.mac();
    assert_ne!(mac, MAC, "not the iMACAddress");
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    let strings =
        calls(&bus).into_iter().filter(|c| matches!(c, Call::In(s, _) if s.value >> 8 == 3));
    assert_eq!(strings.count(), 0, "the iMACAddress string is never read");
    (mac, bus)
}

#[test]
fn a_device_that_takes_set_net_address_is_given_the_drawn_address() {
    let (mac, bus) = bound(&Dev::ncm(CAP_NET_ADDRESS));
    let c = calls(&bus);
    let set = Call::Out(Setup::new(0x21, 0x82, 0, 0), mac.to_vec());
    let at = c.iter().position(|x| *x == set).expect("SET_NET_ADDRESS with the drawn address");
    let on = c.iter().position(|x| *x == Call::Out(Setup::set_interface(1, 1), vec![]));
    assert!(at < on.expect("data alternate"), "sent in alternate setting 0");
    assert_eq!(filter_of(&c), Some(0x000E), "its filter stays closed to other stations");
}

#[test]
fn a_device_without_it_or_that_refuses_it_has_its_filter_opened() {
    for (caps, refuse) in [(0, None), (CAP_NET_ADDRESS, Some((0x82, -32)))] {
        let mut dev = Dev::ncm(caps);
        dev.refuse = refuse;
        let (_, bus) = bound(&dev);
        let c = calls(&bus);
        assert_eq!(filter_of(&c), Some(0x000F), "caps {caps:#x}: promiscuous added");
        let sets = c.iter().filter(|x| matches!(x, Call::Out(s, _) if s.request == 0x82));
        assert_eq!(sets.count(), (caps != 0) as usize, "asked only of a device that offers it");
    }
}
