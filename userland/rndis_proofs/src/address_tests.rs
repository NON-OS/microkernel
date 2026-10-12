// NONOS Operating System (AGPL-3.0-or-later)
//! The station address: drawn for the run, never the device's permanent
//! one, which is not asked for. It is set as OID_802_3_CURRENT_ADDRESS, and
//! the packet filter is opened to every frame whether the device takes it
//! or, as QEMU does, refuses it: Linux's RNDIS_DEFAULT_FILTER, since a
//! device that took the set may still filter on its own address.

use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::{Bind, Nic, Setup, Taken};

use crate::control_tests::bind_with;
use crate::device::{words, NIC_MAC};
use crate::qemu::{honest, Tamper};

fn sent(bus: &MockBus) -> Vec<Vec<u8>> {
    let calls = bus.0.borrow().calls.clone();
    let send = Setup::new(0x21, 0x00, 0, 0);
    calls
        .into_iter()
        .filter_map(|c| matches!(&c, Call::Out(s, _) if *s == send).then_some(c))
        .filter_map(|c| if let Call::Out(_, m) = c { Some(m) } else { None })
        .collect()
}

fn bound(tamper: Tamper) -> ([u8; 6], Vec<Vec<u8>>) {
    let (bus, r) = bind_with(tamper);
    let Bind::Ours(nic) = r else { panic!("not bound") };
    let mac = nic.mac();
    assert_ne!(mac, NIC_MAC, "not the permanent address");
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    let msgs = sent(&bus);
    assert!(msgs.iter().all(|m| m[0] != 4), "no QUERY: the permanent address is never read");
    let mut set = words(&[5, 34, 2, 0x0101_0102, 6, 20, 0]);
    set.extend_from_slice(&mac);
    assert!(msgs.contains(&set), "OID_802_3_CURRENT_ADDRESS set to the drawn address");
    (mac, msgs)
}

fn filter(id: u32, value: u32) -> Vec<u8> {
    words(&[5, 32, id, 0x0001_010E, 4, 20, 0, value])
}

#[test]
fn qemu_refuses_the_drawn_address_and_its_filter_is_opened() {
    let (bus, r) = bind_with(honest());
    assert!(matches!(r, Bind::Ours(ref n) if n.taken() == Taken::Promiscuous));
    assert_eq!(sent(&bus).last(), Some(&filter(3, 0x2D)), "RNDIS_DEFAULT_FILTER");
}

#[test]
fn a_device_that_takes_the_drawn_address_has_its_filter_opened_as_linux_does() {
    let takes: Tamper = Box::new(|k, mut r| {
        if k == 5 {
            r[12..16].fill(0);
        }
        vec![r]
    });
    let (_, msgs) = bound(takes);
    assert_eq!(msgs.last(), Some(&filter(3, 0x2D)), "promiscuous, as RNDIS_DEFAULT_FILTER");
}
