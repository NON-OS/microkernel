// NONOS Operating System (AGPL-3.0-or-later)
//! Binding QEMU's usb-net in its ECM configuration, in Linux's order.

use nonos_usbnet::found::fetch;
use nonos_usbnet::mock::Call;
use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, MAC};
use nonos_usbnet::{Bind, Nic, Setup};

use crate::ecm::bind;
use crate::ecm::function::find_ecm;
use crate::qemu::usb_net;

#[test]
fn the_ecm_function_is_found_and_rndis_is_not_taken_for_it() {
    let f = find_ecm(&CONFIG_ECM).unwrap();
    assert_eq!((f.comm, f.data, f.data_alt, f.mac_index), (0, 1, 1, 3));
    assert_eq!((f.pipes.bulk_in, f.pipes.bulk_out, f.pipes.max_packet_out), (0x82, 0x02, 64));
    assert_eq!(find_ecm(&CONFIG_RNDIS), None);
}

#[test]
fn qemu_usb_net_binds_in_configuration_one_with_its_mac() {
    let bus = usb_net(None);
    let found = fetch(&mut bus.clone(), 2).unwrap();
    bus.0.borrow_mut().calls.clear();
    let Bind::Ours(nic) = bind(bus.clone(), &found) else { panic!("not bound") };
    assert_eq!(nic.mac(), MAC);
    let calls = bus.0.borrow().calls.clone();
    assert!(matches!(calls[0], Call::Configure(p) if p.bulk_in == 0x82 && p.bulk_out == 0x02));
    assert_eq!(calls[1], Call::Out(Setup::set_configuration(1), vec![]));
    assert_eq!(calls[2], Call::Out(Setup::set_interface(1, 1), vec![]));
    assert!(matches!(calls[3], Call::In(s, _) if s.value == 0x0303));
    assert_eq!(calls[4], Call::Out(Setup::new(0x21, 0x43, 0x0E, 0), vec![]));
}

#[test]
fn a_refused_configuration_is_a_named_failure_and_a_refused_filter_is_not() {
    let found = fetch(&mut usb_net(None), 2).unwrap();
    let r = bind(usb_net(Some((0x09, -32))), &found);
    assert!(matches!(r, Bind::Failed("SET_CONFIGURATION refused", -32)));
    assert!(matches!(bind(usb_net(Some((0x43, -32))), &found), Bind::Ours(_)));
}

#[test]
fn devices_without_ecm_and_rtl8153_adapters_are_not_taken() {
    let mut found = fetch(&mut usb_net(None), 2).unwrap();
    found.configs.remove(1);
    assert!(matches!(bind(usb_net(None), &found), Bind::NotOurs));
    let mut found = fetch(&mut usb_net(None), 2).unwrap();
    (found.info.vendor, found.info.product) = (0x0bda, 0x8153);
    assert!(matches!(bind(usb_net(None), &found), Bind::NotOurs));
}
