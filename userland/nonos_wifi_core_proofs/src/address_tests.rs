// NONOS Operating System (AGPL-3.0-or-later)

//! One address per network from the per-boot key: pinned against an
//! independent HMAC-SHA256, the same for one network, different across
//! networks (a name under another security is another network) and boots,
//! and always local and unicast.

use nonos_wifi_core::address::{one_off, AddressKey, Security};

const PSK: Security = Security::Psk;

#[test]
fn the_address_matches_an_independent_derivation() {
    // Python hmac.new([0x5A] * 32, label + bytes([kind]) + b"home", sha256),
    // first six bytes, group bit cleared and local bit set.
    let k = AddressKey::new(&[0x5A; 32]);
    assert_eq!(k.for_network(b"home", Security::Open), [0xAA, 0xBD, 0x29, 0xB2, 0xD6, 0xA5]);
    assert_eq!(k.for_network(b"home", PSK), [0xD2, 0xB7, 0x73, 0xBC, 0x07, 0xF3]);
    assert_eq!(k.for_network(b"home", Security::Sae), [0x82, 0xF5, 0xF6, 0xC1, 0xA5, 0xF2]);
}

#[test]
fn one_name_under_psk_and_under_sae_is_two_networks() {
    let k = AddressKey::new(&[7; 32]);
    let (psk, sae) = (k.for_network(b"home", PSK), k.for_network(b"home", Security::Sae));
    assert_ne!(psk, sae, "a PSK twin of an SAE network does not see its address");
    assert_ne!(k.for_network(b"home", Security::Open), psk);
    assert_ne!(k.for_network(b"home", Security::Open), sae);
}

#[test]
fn one_network_one_address_and_two_networks_two() {
    let k = AddressKey::new(&[7; 32]);
    assert_eq!(k.for_network(b"home", PSK), k.for_network(b"home", PSK), "rejoins match");
    assert_ne!(k.for_network(b"home", PSK), k.for_network(b"work", PSK));
    assert_ne!(k.for_network(b"home", PSK), k.for_network(b"home ", PSK), "every byte counts");
    let next_boot = AddressKey::new(&[8; 32]);
    assert_ne!(k.for_network(b"home", PSK), next_boot.for_network(b"home", PSK));
}

#[test]
fn every_address_is_local_and_unicast() {
    for seed in 0..=255u8 {
        let k = AddressKey::new(&[seed; 32]);
        for ssid in [&b""[..], b"a", &[0xFF; 32]] {
            for kind in [Security::Open, PSK, Security::Sae] {
                assert_eq!(k.for_network(ssid, kind)[0] & 0x03, 0x02, "{seed} {ssid:?}");
            }
        }
    }
}

#[test]
fn a_one_off_address_is_the_draw_made_local_and_unicast() {
    assert_eq!(one_off(&[0xFF; 6]), [0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(one_off(&[0; 6]), [0x02, 0, 0, 0, 0, 0]);
}

#[test]
fn a_station_takes_a_new_address_only_between_associations() {
    use nonos_wifi_core::station::{Ccmp, LinkStation};
    let (bssid, home, work) = ([2, 0, 0, 0, 0, 1], [2, 0, 0, 0, 0, 0xAA], [2, 0, 0, 0, 0, 0xBB]);
    let eth = |src: [u8; 6]| [&[0x33; 6][..], &src, &[0x08, 0x00], b"payload"].concat();
    let mut st = LinkStation::new([2, 0, 0, 0, 0, 2]);
    assert!(st.readdress(home), "unassociated: taken");
    st.associate(bssid, Ccmp::Hardware);
    assert_eq!(&st.tx_frame(&eth(home)).expect("framed")[10..16], &home, "sent from it");
    assert!(!st.readdress(work), "associated: refused");
    assert_eq!(&st.tx_frame(&eth(home)).expect("framed")[10..16], &home, "unchanged");
    st.deassociate();
    assert!(st.readdress(work));
    st.associate(bssid, Ccmp::Hardware);
    assert_eq!(&st.tx_frame(&eth(work)).expect("framed")[10..16], &work);
}
