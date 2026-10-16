// NONOS Operating System (AGPL-3.0-or-later)
//! Keys, and the queues asked for by TID.

use crate::gen3::station::key::{add_key, remove_key, KeyKind};
use crate::gen3::station::queue::{add_queue, cb_size, flush_sta, parse_queue_reply, remove_queue, QueueGiven};
use super::fixtures::image;

#[test]
fn keys_carry_the_cipher_the_group_bit_and_management_protection() {
    let tk = [0x5Au8; 16];
    let mut want = vec![0u8; 80];
    want[0] = 1;
    want[4] = 1;
    want[12] = 0x42; // CCMP, MFP
    want[16..32].copy_from_slice(&tk);
    assert_eq!(add_key(1, 0, KeyKind::Pairwise { mfp: true }, &tk).to_vec(), want);
    want[12] = 0x02;
    assert_eq!(add_key(1, 0, KeyKind::Pairwise { mfp: false }, &tk).to_vec(), want);
    let gtk = [0xA5u8; 16];
    want[8] = 2;
    want[12] = 0x22; // CCMP, multicast
    want[16..32].copy_from_slice(&gtk);
    assert_eq!(add_key(1, 2, KeyKind::Group, &gtk).to_vec(), want);
    let remove = remove_key(1, 2, KeyKind::Group);
    assert_eq!(remove.to_vec(), image(&[3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0x22, 0, 0, 0], 80));
}

#[test]
fn queues_are_asked_for_by_tid_and_their_reply_is_exactly_eight_bytes() {
    let add = add_queue(1, 15, 128, 0x1122_3344_5566_7788, 0x0102_0304_0506_0708);
    let mut want = vec![0u8; 36];
    want[4] = 1;
    want[8] = 15;
    want[16] = 4;
    want[20..28].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    want[28..36].copy_from_slice(&0x0102_0304_0506_0708u64.to_le_bytes());
    assert_eq!(add.to_vec(), want);
    assert_eq!(remove_queue(1, 0).to_vec(), image(&[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], 36));
    assert_eq!((cb_size(128), cb_size(256), cb_size(16)), (4, 5, 1));
    let reply = [9, 0, 0, 0, 0x34, 0x12, 0, 0];
    assert_eq!(parse_queue_reply(&reply), Some(QueueGiven { queue: 9, write_ptr: 0x1234 }));
    assert_eq!(parse_queue_reply(&reply[..7]), None);
    assert_eq!(parse_queue_reply(&[0u8; 12]), None);
    assert_eq!(flush_sta(), [0, 0, 0, 0, 0xFF, 0xFF, 0, 0]);
}
