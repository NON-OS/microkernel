// NONOS Operating System (AGPL-3.0-or-later)
//! Finding the RNDIS function in QEMU usb-net's configuration, and in
//! copies of it changed the ways real devices differ: the interface
//! classes Linux rndis_host matches, a modem's ACM capabilities, and the
//! Union and call management descriptors of phones that get them wrong.

use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS};

use crate::rndis::function::find_rndis;

/// Byte offsets in CONFIG_RNDIS: control interface number and class
/// triple, call management bDataInterface, ACM bmCapabilities, Union
/// master and slave, data interface number and class.
const CTL_NUM: usize = 11;
const CTL_CLASS: usize = 14;
const CM_DATA: usize = 27;
const ACM_CAPS: usize = 31;
const UNION_MASTER: usize = 35;
const UNION_SLAVE: usize = 36;
const DATA_NUM: usize = 46;
const DATA_CLASS: usize = 49;

fn with(changes: &[(usize, u8)]) -> Vec<u8> {
    let mut raw = CONFIG_RNDIS.to_vec();
    for &(at, v) in changes {
        raw[at] = v;
    }
    raw
}

fn data_of(raw: &[u8]) -> Option<(u8, u8)> {
    find_rndis(raw).map(|f| (f.comm, f.data))
}

#[test]
fn qemu_usb_net_rndis_is_found_with_its_bulk_pipes_and_ecm_is_not() {
    let f = find_rndis(&CONFIG_RNDIS).unwrap();
    assert_eq!((f.comm, f.data, f.data_alt), (0, 1, 0));
    assert_eq!((f.pipes.bulk_in, f.pipes.bulk_out, f.pipes.max_packet_out), (0x82, 0x02, 64));
    assert_eq!(find_rndis(&CONFIG_ECM), None);
}

#[test]
fn the_three_rndis_classes_match_and_activesync_and_plain_acm_do_not() {
    let class =
        |c: [u8; 3]| with(&[(CTL_CLASS, c[0]), (CTL_CLASS + 1, c[1]), (CTL_CLASS + 2, c[2])]);
    assert!(find_rndis(&class([0xE0, 0x01, 0x03])).is_some(), "wireless RNDIS");
    assert!(find_rndis(&class([0xEF, 0x04, 0x01])).is_some(), "misc RNDIS");
    assert!(find_rndis(&class([0xEF, 0x01, 0x01])).is_none(), "ActiveSync");
    assert!(find_rndis(&class([0x02, 0x02, 0x01])).is_none(), "an AT modem");
}

#[test]
fn acm_capabilities_mark_a_modem_except_on_wireless_class_rndis() {
    assert!(find_rndis(&with(&[(ACM_CAPS, 0x02)])).is_none());
    let wireless =
        [(ACM_CAPS, 0x02), (CTL_CLASS, 0xE0), (CTL_CLASS + 1, 0x01), (CTL_CLASS + 2, 0x03)];
    assert!(find_rndis(&with(&wireless)).is_some());
}

#[test]
fn a_union_is_trusted_and_a_broken_one_falls_back_as_far_as_interface_one() {
    let moved = [(CTL_NUM, 2), (DATA_NUM, 3), (UNION_MASTER, 2), (UNION_SLAVE, 3), (CM_DATA, 3)];
    let mut raw = with(&moved);
    assert_eq!(data_of(&raw), Some((2, 3)), "Union");
    raw[UNION_SLAVE] = 9;
    assert_eq!(data_of(&raw), Some((2, 3)), "call management");
    raw[CM_DATA] = 9;
    assert_eq!(data_of(&raw), None, "no guess past a control interface 0");
    assert_eq!(data_of(&with(&[(UNION_SLAVE, 9), (CM_DATA, 9)])), Some((0, 1)), "Linux's 0 and 1");
    assert_eq!(data_of(&with(&[(DATA_CLASS, 0xFF)])), None, "a Union naming a non-data one");
}
