// NONOS Operating System (AGPL-3.0-or-later)

//! A received frame classified, and an EAPOL payload wrapped.

use nonos_wifi_core::dot11::data::{build_data, parse_data};

use super::beacons::{AP, OUR};
use crate::assoc::{classify, wrap_eapol, RxKind};

#[test]
fn a_management_frame_is_classified_as_management() {
    // Frame control type bits = management (0). A beacon frame control is 0x0080.
    let frame = [0x80u8, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut eth = Vec::new();
    assert_eq!(classify(&frame, &mut eth), RxKind::Mgmt);
}

#[test]
fn an_eapol_data_frame_is_unwrapped_to_its_payload() {
    // Build an ethernet frame carrying an EAPOL payload, wrap it as an 802.11 data
    // frame, and confirm classify pulls the payload back out.
    let eapol = [0x02u8, 0x03, 0x00, 0x5F, 0xAB, 0xCD]; // EAPOL version/type + body
    let mut eth = Vec::new();
    eth.extend_from_slice(&OUR); // dst
    eth.extend_from_slice(&AP); // src
    eth.extend_from_slice(&0x888Eu16.to_be_bytes());
    eth.extend_from_slice(&eapol);
    let mpdu = build_data(&eth, OUR, AP, 0).unwrap();

    let mut out = Vec::new();
    match classify(&mpdu, &mut out) {
        RxKind::Eapol(start, end) => assert_eq!(&out[start..end], &eapol),
        other => panic!("expected EAPOL, got {other:?}"),
    }
}

#[test]
fn a_non_eapol_data_frame_is_ignored() {
    // A data frame carrying IPv4 (ethertype 0x0800) is not part of the join.
    let mut eth = Vec::new();
    eth.extend_from_slice(&OUR);
    eth.extend_from_slice(&AP);
    eth.extend_from_slice(&0x0800u16.to_be_bytes());
    eth.extend_from_slice(&[1, 2, 3, 4]);
    let mpdu = build_data(&eth, OUR, AP, 0).unwrap();
    let mut out = Vec::new();
    assert_eq!(classify(&mpdu, &mut out), RxKind::Other);
}

#[test]
fn wrap_eapol_builds_a_data_frame_to_the_ap() {
    let eapol = [0x02u8, 0x03, 0x00, 0x10, 0x99];
    let mpdu = wrap_eapol(&eapol, OUR, AP, 7).unwrap();
    // It round-trips: parse the data frame back to ethernet and recover the EAPOL.
    let eth = parse_data(&mpdu).unwrap();
    assert_eq!(&eth[0..6], &AP, "destination is the AP");
    assert_eq!(&eth[6..12], &OUR, "source is the station");
    assert_eq!(u16::from_be_bytes([eth[12], eth[13]]), 0x888E, "ethertype is EAPOL");
    assert_eq!(&eth[14..], &eapol, "payload preserved");
}
