// NONOS Operating System (AGPL-3.0-or-later)
//! Every command payload has its Linux shape, and the MCC reply is read by
//! its version.

use crate::gen3::cmds::*;

#[test]
fn every_command_payload_has_its_linux_shape() {
    assert_eq!(init_extended_cfg(), [2, 0, 0, 0]);
    assert_eq!(reserved_word(), [0; 4]);
    assert_eq!(tx_ant_cfg(3), [3, 0, 0, 0]);
    assert_eq!(bt_config(true), vec![1, 0, 0, 0, 0x15, 0, 0, 0], "NW, MPLUT, SYNC2SCO, high-band");
    assert_eq!(bt_config(false)[4], 0x14);
    let soc = soc_configuration(true, 2, true, 12000);
    assert_eq!(u32::from_le_bytes(soc[0..4].try_into().unwrap()), (2 << 2) | 2, "LTR 2500 us, low latency");
    assert_eq!(u32::from_le_bytes(soc[4..8].try_into().unwrap()), 12000);
    assert_eq!(soc_configuration(false, 0, false, 0)[0], 1, "discrete");
    assert_eq!(device_power(), [0; 4], "CAM");
    let mcc = mcc_update_world();
    assert_eq!((mcc.len(), u16::from_le_bytes([mcc[0], mcc[1]]), mcc[2]), (28, 0x5A5A, 0), "ZZ, from the firmware");
    let sc = scan_config(3, 1);
    assert_eq!((sc.len(), sc[2], sc[4], sc[8]), (12, 0, 3, 1));
    let mac = mac_config_add([2, 0, 0, 0, 0, 1]);
    assert_eq!(u32::from_le_bytes(mac[4..8].try_into().unwrap()), 1, "add");
    assert_eq!(u32::from_le_bytes(mac[8..12].try_into().unwrap()), 5, "BSS station");
    assert_eq!(&mac[12..18], &[2, 0, 0, 0, 0, 1]);
    assert_eq!(u32::from_le_bytes(mac[20..24].try_into().unwrap()), 0b1100, "group frames and beacons");
    assert_eq!(u32::from_le_bytes(mac[32..36].try_into().unwrap()), 1, "no ACK-enabled aggregation");
    assert!(mac[36..].iter().all(|&b| b == 0), "unassociated");
    let link = link_config_add([2, 0, 0, 0, 0, 1]);
    assert_eq!(link.len(), 208);
    assert_eq!(u32::from_le_bytes(link[0..4].try_into().unwrap()), 1);
    assert_eq!(u32::from_le_bytes(link[12..16].try_into().unwrap()), 0xFFFF_FFFF, "no PHY context");
    assert_eq!(&link[16..22], &[2, 0, 0, 0, 0, 1]);
}

#[test]
fn the_mcc_reply_is_read_by_its_version_and_its_length_checked() {
    let mut v4 = vec![0u8; 16 + 4 + 8];
    v4[4..6].copy_from_slice(&0x4445u16.to_le_bytes());
    v4[16..20].copy_from_slice(&2u32.to_le_bytes());
    assert_eq!(mcc_update_reply(&v4, 6, true), Some((0, 0x4445)));
    assert_eq!(mcc_update_reply(&v4[..27], 6, true), None, "two channels promised, one and a half given");
    let mut v3 = vec![0u8; 16];
    v3[0] = 1;
    assert_eq!(mcc_update_reply(&v3, 5, false), Some((1, 0)));
    let v8 = vec![0u8; 24];
    assert!(mcc_update_reply(&v8, 8, true).is_some());
    let mut huge = vec![0u8; 20];
    huge[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(mcc_update_reply(&huge, 6, true), None, "a channel count past the reply");
}
