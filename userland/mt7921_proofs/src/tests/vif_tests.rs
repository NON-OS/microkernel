// NONOS Operating System (AGPL-3.0-or-later)

//! The interface's command bodies, offset by offset against the Linux
//! v6.12 structs: `DEV_INFO_ACTIVE`, the BSS basic TLV for a station (and
//! the UNI meaning of its connection state), QoS and RLM.

use crate::vif::bss::{basic, rlm, Basic};
use crate::vif::dev;

const ADDR: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];

#[test]
fn the_device_entry_names_its_address_and_whether_it_is_active() {
    let b = dev::body(true, ADDR);
    assert_eq!(&b[..10], &[0, 0, 0, 0, 0, 0, 12, 0, 1, 0], "omac 0, band 0, tag 0, len 12, on");
    assert_eq!(&b[10..], &ADDR);
    assert_eq!(dev::body(false, ADDR)[8], 0);
}

#[test]
fn the_bss_basic_tlv_is_a_stations_and_zero_means_connected() {
    let up = Basic {
        active: true,
        connected: true,
        bssid: ADDR,
        beacon_int: 100,
        dtim: 1,
        phymode: 6,
        nonht_basic_phy: 3,
    };
    let (b, n) = basic(up, Some(false));
    assert_eq!(n, 44);
    let t = &b[4..36];
    assert_eq!(&t[..8], &[0, 0, 32, 0, 1, 0, 0, 0], "tag 0, len 32, active, omac, hw bss, band");
    assert_eq!(u32::from_le_bytes(t[8..12].try_into().unwrap()), 0x1_0001, "CONNECTION_INFRA_STA");
    assert_eq!((t[12], t[13]), (0, 0), "connected, WMM set 0");
    assert_eq!(&t[14..20], &ADDR);
    assert_eq!((t[20], t[22], t[24], t[25], t[26], t[28]), (19, 100, 1, 6, 19, 3));
    assert_eq!(&b[36..44], &[15, 0, 8, 0, 0, 0, 0, 0], "QBSS, no QoS");
    let (down, n) = basic(Basic { active: true, ..Default::default() }, None);
    assert_eq!((n, down[16]), (36, 1), "the interface's own BSS: not connected");
}

#[test]
fn the_rlm_tlv_names_the_20_mhz_channel() {
    let b = rlm(36, 1, 0x3);
    assert_eq!(&b[4..8], &[2, 0, 16, 0]);
    assert_eq!(
        &b[8..18],
        &[36, 36, 0, 0, 2, 3, 1, 0, 0, 1],
        "ctrl, centre, -, 20, tx, rx, short slot"
    );
}
