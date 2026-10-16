// NONOS Operating System (AGPL-3.0-or-later)

//! One address per network from the per-boot key: pinned against an
//! independent HMAC-SHA256, the same for one network, different across
//! networks and boots, and always local and unicast.

use nonos_wifi_core::address::AddressKey;

#[test]
fn the_address_matches_an_independent_derivation() {
    // Python hmac.new([0x5A] * 32, label + b"home", sha256), first six bytes,
    // group bit cleared and local bit set.
    let k = AddressKey::new(&[0x5A; 32]);
    assert_eq!(k.for_network(b"home"), [0x12, 0xA5, 0x21, 0x69, 0xAE, 0x1D]);
}

#[test]
fn one_network_one_address_and_two_networks_two() {
    let k = AddressKey::new(&[7; 32]);
    assert_eq!(k.for_network(b"home"), k.for_network(b"home"), "rejoins match");
    assert_ne!(k.for_network(b"home"), k.for_network(b"work"));
    assert_ne!(k.for_network(b"home"), k.for_network(b"home "), "every byte counts");
    let next_boot = AddressKey::new(&[8; 32]);
    assert_ne!(k.for_network(b"home"), next_boot.for_network(b"home"));
}

#[test]
fn every_address_is_local_and_unicast() {
    for seed in 0..=255u8 {
        let k = AddressKey::new(&[seed; 32]);
        for ssid in [&b""[..], b"a", &[0xFF; 32]] {
            assert_eq!(k.for_network(ssid)[0] & 0x03, 0x02, "{seed} {ssid:?}");
        }
    }
}
