// NONOS Operating System (AGPL-3.0-or-later)

//! Received packets and ALIVE, read within their bounds.

use crate::gen3::alive::{parse as parse_alive, STATUS_OK};
use crate::gen3::packet::parse as parse_packet;

#[test]
fn a_received_packet_is_read_only_within_its_buffer() {
    let mut rb = vec![0u8; 64];
    rb[0..4].copy_from_slice(&8u32.to_le_bytes());
    rb[4] = 0x01;
    rb[5] = 0x00;
    rb[6..8].copy_from_slice(&0x8000u16.to_le_bytes());
    rb[8..12].copy_from_slice(&[1, 2, 3, 4]);
    let p = parse_packet(&rb).expect("parses");
    assert_eq!((p.cmd, p.group, p.sequence, p.payload), (1, 0, 0x8000, &[1u8, 2, 3, 4][..]));
    assert!(!p.is_reply(), "the firmware originated it");
    rb[0..4].copy_from_slice(&61u32.to_le_bytes());
    assert!(parse_packet(&rb).is_none(), "longer than the buffer");
    rb[0..4].copy_from_slice(&3u32.to_le_bytes());
    assert!(parse_packet(&rb).is_none(), "shorter than its header");
    rb[0..4].copy_from_slice(&0x5555_0000u32.to_le_bytes());
    assert!(parse_packet(&rb).is_none(), "the end marker");
    assert!(parse_packet(&rb[..3]).is_none());
}

#[test]
fn alive_is_read_by_version_and_refused_when_short() {
    let mut v6 = vec![0u8; 144];
    v6[0..2].copy_from_slice(&STATUS_OK.to_le_bytes());
    v6[20..24].copy_from_slice(&0x0040_1000u32.to_le_bytes()); // LMAC error table
    v6[40..44].copy_from_slice(&0x00A0_2000u32.to_le_bytes()); // SCD base
    v6[100..104].copy_from_slice(&0x4Au32.to_le_bytes());
    v6[104..108].copy_from_slice(&0x1Fu32.to_le_bytes());
    v6[108..112].copy_from_slice(&0xC040_0000u32.to_le_bytes()); // with cache bits
    v6[116..120].copy_from_slice(&7u32.to_le_bytes());
    let a = parse_alive(&v6, Some(6)).expect("v6");
    assert!(a.ok());
    assert_eq!((a.umac_major, a.umac_minor), (0x4A, 0x1F));
    assert_eq!(a.lmac_error_table, 0x0040_1000);
    assert_eq!(a.scd_base, 0x00A0_2000);
    assert_eq!(a.umac_error_table, 0x0040_0000, "FW_ADDR_CACHE_CONTROL masked off");
    assert_eq!(a.sku_id, [7, 0, 0]);
    assert!(parse_alive(&v6[..143], Some(6)).is_none(), "shorter than version 6");
    assert!(parse_alive(&v6[..128], Some(5)).is_some(), "version 5 is 128 bytes");
    let v4 = &v6[..116];
    assert_eq!(parse_alive(v4, None).map(|a| a.sku_id), Some([0; 3]), "version 4 by its size");
    let v3 = &v6[..68];
    assert!(parse_alive(v3, None).is_some(), "version 3 by its size");
    assert!(parse_alive(&v6[..100], None).is_none(), "an unknown size");
    let mut dead = v6.clone();
    dead[0..2].copy_from_slice(&0xDEADu16.to_le_bytes());
    assert!(!parse_alive(&dead, Some(6)).unwrap().ok());
}
