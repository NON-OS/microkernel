// NONOS Operating System (AGPL-3.0-or-later)

//! Command framing, byte for byte against `mt76_connac2_mcu_fill_message`:
//! the hardware TXD's length, packet format and queue; the unified header
//! (`cid`, packet id, sequence, `MCU_CMD_UNI_EXT_ACK`); the other header
//! (`pq_id`, `cid`, `set_query`, `ext_cid`, its ack).

use crate::mcu::cmd::*;
use crate::mcu::frame::{frame, HDR_LEN, UNI_HDR_LEN};

fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

#[test]
fn a_unified_command_carries_its_cid_and_the_ack_option() {
    let mut out = [0u8; 128];
    let n = frame(STA_REC_UPDATE, 9, &[1, 2, 3], &mut out).unwrap();
    assert_eq!((n, UNI_HDR_LEN), (51, 48));
    assert_eq!(word(&out, 0), 51 | 2 << 23 | 0x20 << 25, "length, MT_TX_TYPE_CMD, RX_Q0");
    assert_eq!(word(&out, 4), 1 << 31 | 1 << 16, "long format, MT_HDR_FORMAT_CMD");
    assert_eq!(u16::from_le_bytes([out[32], out[33]]), 19, "length past the TXD");
    assert_eq!((out[34], out[35], out[37], out[39]), (0x03, 0, 0xa0, 9));
    assert_eq!((out[42], out[43]), (0, 0x07), "to the N9, ack | uni | set");
    assert_eq!(&out[48..51], &[1, 2, 3]);
}

#[test]
fn an_extended_command_rides_under_ext_cid_and_asks_an_ack() {
    let mut out = [0u8; 128];
    let n = frame(SET_RX_PATH, 3, &[7; 4], &mut out).unwrap();
    assert_eq!((n, HDR_LEN), (68, 64));
    assert_eq!(u16::from_le_bytes([out[34], out[35]]), 0x8000, "pq_id");
    assert_eq!((out[36], out[37], out[38], out[39]), (0xed, 0xa0, 1, 3), "cid, id, SET, seq");
    assert_eq!((out[41], out[42], out[43]), (0x4e, 0, 1), "ext_cid, H2N, ext_cid_ack");
}

#[test]
fn offload_and_download_commands_differ_only_in_set_query() {
    let mut out = [0u8; 128];
    frame(GET_NIC_CAPAB, 1, &[], &mut out).unwrap();
    assert_eq!((out[36], out[38], out[41], out[43]), (0x8a, 1, 0, 0), "CE: MCU_Q_SET");
    frame(PATCH_SEM_CONTROL, 2, &[1, 0, 0, 0], &mut out).unwrap();
    assert_eq!((out[36], out[38], out[41], out[43]), (0x10, 3, 0, 0), "MCU_Q_NA");
}

#[test]
fn a_command_too_long_for_its_buffer_or_sixteen_bits_is_not_framed() {
    let mut out = [0u8; 70];
    assert!(frame(SET_RX_PATH, 1, &[0; 7], &mut out).is_none());
    assert!(frame(SET_RX_PATH, 1, &[0; 6], &mut out).is_some());
    let mut big = vec![0u8; 70_000];
    assert!(frame(STA_REC_UPDATE, 1, &vec![0; 65_536 - 48], &mut big).is_none());
}
