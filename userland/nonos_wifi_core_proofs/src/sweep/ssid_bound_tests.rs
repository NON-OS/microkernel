// NONOS Operating System (AGPL-3.0-or-later)
//! An SSID element longer than 32 octets (IEEE Std 802.11-2020, 9.4.2.2) makes
//! the frame no beacon: found by the parser fuzz, refused by `parse_beacon`.

use nonos_wifi_core::dot11::parse::{parse_beacon, SSID_LEN_MAX};

fn beacon(ssid: &[u8]) -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0];
    f.extend_from_slice(&[0xFF; 6]);
    f.extend_from_slice(&[2, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 1, 0, 0]);
    f.extend_from_slice(&[0; 12]);
    f.extend_from_slice(&[0, ssid.len() as u8]);
    f.extend_from_slice(ssid);
    f.extend_from_slice(&[3, 1, 6]);
    f
}

#[test]
fn an_ssid_over_32_octets_is_no_beacon() {
    let longest = [b'a'; SSID_LEN_MAX];
    let b = beacon(&longest);
    assert_eq!(parse_beacon(&b).map(|i| (i.ssid.len(), i.channel)), Some((32, 6)));
    assert!(parse_beacon(&beacon(&[b'a'; SSID_LEN_MAX + 1])).is_none());
    assert!(parse_beacon(&beacon(&[b'a'; 255])).is_none());
}
