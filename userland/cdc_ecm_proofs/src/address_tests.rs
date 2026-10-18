// NONOS Operating System (AGPL-3.0-or-later)
//! The station address: drawn for the run, never the iMACAddress the
//! device carries. ECM has no request that gives the device another
//! address, so its packet filter is opened to every frame instead. A
//! segment size under a frame is refused; a large one is taken.

use nonos_usbnet::found::fetch;
use nonos_usbnet::mock::{descriptors, drawn, Call, MockBus};
use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE, MAC, MAC_STRING};
use nonos_usbnet::{Bind, Nic};

use crate::device::bound;
use crate::ecm::bind;
use crate::qemu::usb_net;

fn filter_of(calls: &[Call]) -> Option<u16> {
    calls.iter().find_map(|c| match c {
        Call::Out(s, _) if s.request == 0x43 => Some(s.value),
        _ => None,
    })
}

#[test]
fn the_address_on_the_wire_is_drawn_and_the_filter_opened_for_it() {
    let bus = usb_net(None);
    let found = fetch(&mut bus.clone(), 2).unwrap();
    bus.0.borrow_mut().calls.clear();
    let Bind::Ours(nic) = bind(bus.clone(), &found, drawn()) else { panic!("not bound") };
    let mac = nic.mac();
    assert_ne!(mac, MAC, "not the iMACAddress");
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    let calls = bus.0.borrow().calls.clone();
    let strings = calls.iter().filter(|c| matches!(c, Call::In(s, _) if s.value >> 8 == 3));
    assert_eq!(strings.count(), 0, "the iMACAddress string is never read");
    assert_eq!(filter_of(&calls), Some(0x000F), "directed, multicast, broadcast, promiscuous");
}

#[test]
fn a_device_whose_mac_string_is_blank_or_broken_binds_all_the_same() {
    let bad: [u8; 6] = [0x1a, 0x03, b'4', 0, b'g', 0];
    let bus = MockBus::new(Box::new(move |call| {
        let configs: [&[u8]; 2] = [&CONFIG_RNDIS, &CONFIG_ECM];
        Ok(descriptors(call, &DEVICE, &configs, &[(3, &bad)]).unwrap_or_default())
    }));
    let found = fetch(&mut bus.clone(), 2).unwrap();
    assert!(matches!(bind(bus, &found, drawn()), Bind::Ours(_)));
}

#[test]
fn a_segment_size_under_one_frame_is_refused_and_a_large_one_taken() {
    let size = |n: u16| move |c: &mut Vec<u8>| c[36..38].copy_from_slice(&n.to_le_bytes());
    for n in [0, 1500, 1513] {
        let e = bound(size(n), &MAC_STRING).err();
        assert_eq!(e.as_deref(), Some("segment size under one frame -5"), "{n}");
    }
    assert!(bound(size(1514), &MAC_STRING).is_ok());
    assert!(bound(size(9014), &MAC_STRING).is_ok());
}
