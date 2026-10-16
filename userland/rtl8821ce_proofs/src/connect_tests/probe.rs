// NONOS Operating System (AGPL-3.0-or-later)

//! Only a hidden network is probed, and only by its own name.

use crate::connect_probe::{hunt_probe, PROBE_RATES};

#[test]
fn only_a_hidden_network_is_probed_and_only_by_its_own_name() {
    // A per-boot address: locally administered, unicast.
    let mac = [0x06, 0x11, 0x22, 0x33, 0x44, 0x55];
    let mut out = [0u8; 64];
    for step in 0..13 {
        assert_eq!(hunt_probe(&mut out, false, mac, b"Home", step), None, "a named network is heard, not asked for");
    }
    let n = hunt_probe(&mut out, true, mac, b"Home", 3).expect("a hidden network gets a directed probe");
    let f = &out[..n];
    assert_eq!(f[0], 0x40, "a probe request");
    assert_eq!(&f[4..10], &[0xFF; 6], "to broadcast");
    assert_eq!(&f[10..16], &mac, "from the per-boot address");
    assert_eq!(&f[24..30], &[0, 4, b'H', b'o', b'm', b'e'], "naming the hidden network and nothing else");
    assert_eq!((f[30], f[31]), (1, PROBE_RATES.len() as u8), "then the rates");
    assert_eq!(n, 32 + PROBE_RATES.len());
    // A buffer that cannot hold the frame gets nothing, not a cut frame.
    assert_eq!(hunt_probe(&mut [0u8; 30], true, mac, b"Home", 0), None);
}
