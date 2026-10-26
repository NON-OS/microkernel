// NONOS Operating System (AGPL-3.0-or-later)
//! The station's power command against `struct iwl_mac_power_cmd` and Linux's
//! `iwl_mvm_power_build_cmd`: 40 bytes, the keep-alive period three DTIMs and
//! at least 25 s, power save always allowed, and power management with the
//! 100 ms timeouts only when managed.

use crate::gen3::station::power::{keep_alive_s, mac_power, MAC_POWER_LEN};

#[test]
fn the_power_command_has_its_linux_shape() {
    assert_eq!(MAC_POWER_LEN, 40);
    assert_eq!(keep_alive_s(1, 100), 25, "0.31 s of DTIMs: the 25 s floor");
    assert_eq!(keep_alive_s(10, 1000), 31, "3 x 10 x 1000 TU = 30.72 s, rounded up");
    assert_eq!(keep_alive_s(255, u16::MAX), 51_338, "the largest DTIM interval fits");
    let awake = mac_power(25, false);
    assert_eq!(&awake[..8], &[0, 0, 0, 0, 1, 0, 25, 0], "MAC 0, POWER_SAVE_ENA, 25 s");
    assert!(awake[8..].iter().all(|&b| b == 0), "no timeouts while power management is off");
    let managed = mac_power(31, true);
    assert_eq!(&managed[4..8], &[3, 0, 31, 0], "and POWER_MANAGEMENT_ENA");
    let t = |at: usize| u32::from_le_bytes(managed[at..at + 4].try_into().unwrap());
    assert_eq!((t(8), t(12), t(16), t(20)), (100_000, 100_000, 0, 0), "rx, tx; no uAPSD");
    assert!(managed[24..].iter().all(|&b| b == 0), "no LPRX, DTIM skip, snooze or uAPSD");
}
