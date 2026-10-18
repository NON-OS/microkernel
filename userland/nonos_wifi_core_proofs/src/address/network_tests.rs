// NONOS Operating System (AGPL-3.0-or-later)

//! One address per network from the per-boot key: pinned against an
//! independent HMAC-SHA256, the same for one network, different across
//! networks (a name under another security is another network) and boots,
//! and always local and unicast.

use nonos_wifi_core::address::{one_off, Addresses, Security};

/// The per-boot key, filled with `b`.
fn key(b: u8) -> Addresses {
    Addresses::new(|k| {
        k.fill(b);
        true
    })
    .unwrap()
}

const PSK: Security = Security::Psk;

#[test]
fn the_address_matches_an_independent_derivation() {
    // Python hmac.new([0x5A] * 32, b"join" + b"home" + bytes([kind]), sha256),
    // first six bytes, group bit cleared and local bit set.
    let k = key(0x5A);
    assert_eq!(k.for_network(b"home", Security::Open), [0xF6, 0x44, 0x4E, 0x40, 0x17, 0x6B]);
    assert_eq!(k.for_network(b"home", PSK), [0x9E, 0x14, 0x48, 0x4E, 0x38, 0x0D]);
    assert_eq!(k.for_network(b"home", Security::Sae), [0xFE, 0x02, 0xB5, 0x2D, 0x14, 0x74]);
}

#[test]
fn one_name_under_psk_and_under_sae_is_two_networks() {
    let k = key(7);
    let (psk, sae) = (k.for_network(b"home", PSK), k.for_network(b"home", Security::Sae));
    assert_ne!(psk, sae, "a PSK twin of an SAE network does not see its address");
    assert_ne!(k.for_network(b"home", Security::Open), psk);
    assert_ne!(k.for_network(b"home", Security::Open), sae);
}

#[test]
fn one_network_one_address_and_two_networks_two() {
    let k = key(7);
    assert_eq!(k.for_network(b"home", PSK), k.for_network(b"home", PSK), "rejoins match");
    assert_ne!(k.for_network(b"home", PSK), k.for_network(b"work", PSK));
    assert_ne!(k.for_network(b"home", PSK), k.for_network(b"home ", PSK), "every byte counts");
    let next_boot = key(8);
    assert_ne!(k.for_network(b"home", PSK), next_boot.for_network(b"home", PSK));
}

#[test]
fn every_address_is_local_and_unicast() {
    for seed in 0..=255u8 {
        let k = key(seed);
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
