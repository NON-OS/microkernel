// NONOS Operating System (AGPL-3.0-or-later)
//! The key commands' bytes against rtw89: a security CAM entry
//! (`rtw89_cam_get_sec_key_cmd`), the address CAM entry's key slots
//! (`rtw89_cam_fill_addr_cam_info`, normal mode: 0-1 unicast, 2-4 group,
//! 5-6 BIP), and the WD's security word (`rtw89_build_txwd_info2`); and a
//! key command's body wiped once it is in the ring.

use crate::h2c::cam::cam_body;
use crate::h2c::cam_keys::Keys;
use crate::h2c::media::Net;
use crate::h2c::sec_cam::{sec_body, SEC_UPD};
use crate::tx::classify::classify;
use crate::tx::wd::build;

fn w(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap())
}

#[test]
fn a_security_cam_entry_is_its_index_length_type_and_key() {
    let key: [u8; 16] = core::array::from_fn(|i| i as u8);
    let b = sec_body(5, 10, &key);
    assert_eq!((SEC_UPD.cat, SEC_UPD.class, SEC_UPD.func), (1, 0xa, 1));
    assert_eq!((w(&b, 0), w(&b, 1)), (5 | 20 << 16, 10), "entry 5, offset 0, length 20; BIP");
    assert_eq!(w(&b, 2), 0x0302_0100, "key octets as little-endian words");
    assert_eq!(&b[8..], &key);
}

#[test]
fn keys_take_their_slots_and_the_cam_words_name_them() {
    assert_eq!(Keys::slot(true, 0), Some(0));
    assert_eq!((Keys::slot(false, 1), Keys::slot(false, 3)), (Some(2), Some(4)), "group");
    assert_eq!((Keys::slot(false, 4), Keys::slot(false, 5)), (Some(5), Some(6)), "IGTK");
    assert_eq!((Keys::slot(true, 1), Keys::slot(false, 6)), (None, None));
    let mut k = Keys::default();
    (k.set(0, 0), k.set(3, 2), k.set(5, 4));
    let b = cam_body(&[2; 6], &[4; 6], Net::Infra, 7, k);
    assert_eq!(w(&b, 9), 7 | 2 << 16 | 2 << (18 + 6), "aid, normal mode, slot 3's index 2");
    assert_eq!(w(&b, 10), 0b10_1001, "valid: slots 0, 3, 5; slot 0 entry 0");
    assert_eq!(w(&b, 11), 3 | 5 << 16, "slot 3 on entry 3, slot 5 on entry 5 (IGTK 4 & 3 = 0)");
}

#[test]
fn a_frame_for_the_chip_names_ccmp_hw_enc_and_its_entry() {
    let mut f = vec![0x08, 0x41];
    f.resize(24 + 8 + 10, 0);
    let mut d = classify(&f).unwrap();
    assert_eq!(w(&build(&d), 8), 0, "no key: nothing asked of the chip");
    d.sec = Some(0);
    assert_eq!(w(&build(&d), 8), 6 << 9 | 1 << 8, "CCMP-128, HW_ENC, entry 0");
}

#[test]
fn a_key_commands_body_is_wiped_once_in_the_ring() {
    let (_card, mut chip) = crate::model::rig::ready();
    let mut b = sec_body(0, 6, &[7; 16]);
    assert!(chip.sec_write(&mut b));
    assert_eq!(b, [0; 24]);
}
