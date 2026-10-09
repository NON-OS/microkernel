// NONOS Operating System (AGPL-3.0-or-later)
//! The descriptor walk over QEMU usb-net's configurations, and over bytes a
//! hostile device could send.

use crate::desc::{config_header, device_info, CLASS_CDC_DATA, CLASS_COMM};
use crate::desc::{find_functional, interfaces, mac_from_string, union_data, walk};
use crate::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE, MAC, MAC_STRING};

#[test]
fn device_descriptor_names_netchip_with_two_configurations() {
    let info = device_info(&DEVICE).unwrap();
    assert_eq!((info.vendor, info.product, info.configs), (0x0525, 0xa4a2, 2));
    assert_eq!(config_header(&CONFIG_ECM), Some((80, 1)));
    assert_eq!(config_header(&CONFIG_RNDIS), Some((67, 2)));
}

#[test]
fn ecm_data_interface_has_its_pipes_only_on_alternate_one() {
    let ifs = interfaces(&CONFIG_ECM);
    assert_eq!(ifs.len(), 3);
    assert_eq!((ifs[0].class, ifs[0].subclass), (CLASS_COMM, 0x06));
    assert!(!ifs[0].has_bulk_pair(), "the interrupt endpoint is not a bulk pipe");
    assert_eq!((ifs[1].number, ifs[1].alt, ifs[1].has_bulk_pair()), (1, 0, false));
    let on = ifs[2];
    assert_eq!((on.number, on.alt, on.class), (1, 1, CLASS_CDC_DATA));
    assert_eq!((on.pipes.bulk_in, on.pipes.bulk_out), (0x82, 0x02));
    assert_eq!((on.pipes.max_packet_in, on.pipes.max_packet_out), (64, 64));
}

#[test]
fn functional_descriptors_follow_their_interface() {
    assert_eq!(union_data(&CONFIG_ECM, 0), Some(1));
    let eth = find_functional(&CONFIG_ECM, 0, 0x0F).unwrap();
    assert_eq!((eth[3], u16::from_le_bytes([eth[8], eth[9]])), (3, 1514));
    assert_eq!(union_data(&CONFIG_RNDIS, 0), Some(1));
    assert!(find_functional(&CONFIG_ECM, 1, 0x06).is_none());
}

#[test]
fn mac_string_reads_and_bad_ones_do_not() {
    assert_eq!(mac_from_string(&MAC_STRING), Some(MAC));
    let mut bad = MAC_STRING;
    bad[4] = b'g';
    assert_eq!(mac_from_string(&bad), None);
    let mut group = MAC_STRING;
    group[4] = b'1';
    assert_eq!(mac_from_string(&group), None, "a multicast address is no station");
    assert_eq!(mac_from_string(&MAC_STRING[..20]), None);
}

#[test]
fn a_length_running_past_the_end_or_under_two_stops_the_walk() {
    assert_eq!(walk(&[9, 2, 0]).count(), 0);
    assert_eq!(walk(&[2, 1, 0, 4]).count(), 1);
    assert_eq!(walk(&[0, 0, 0]).count(), 0);
    for cut in 0..CONFIG_ECM.len() {
        let _ = interfaces(&CONFIG_ECM[..cut]);
    }
}

#[test]
fn superspeed_companion_sets_the_burst_of_its_own_pipe() {
    let raw = [
        9, 4, 0, 0, 2, 0xff, 0, 0, 0, // vendor interface
        7, 5, 0x81, 2, 0, 4, 0, 6, 0x30, 3, 0, 0, 0, // bulk IN, burst 3
        7, 5, 0x02, 2, 0, 4, 0, 6, 0x30, 15, 0, 0, 0, // bulk OUT, burst 15
    ];
    let p = interfaces(&raw)[0].pipes;
    assert_eq!((p.burst_in, p.burst_out, p.max_packet_in), (3, 15, 1024));
}
