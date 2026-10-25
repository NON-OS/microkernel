// NONOS Operating System (AGPL-3.0-or-later)

//! What a join keeps for power save: the AID only from a successful
//! association response from the access point itself, and the DTIM period
//! from its beacon's TIM element.

use crate::assoc::facts::{dtim_period, granted_aid};

const AP: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

fn assoc_resp(from: [u8; 6], status: u16, aid: u16) -> Vec<u8> {
    let head = [&[0x10, 0, 0, 0][..], &[0x02; 6], &from, &from, &[0, 0]].concat();
    [head, vec![0x11, 0], status.to_le_bytes().to_vec(), (aid | 0xC000).to_le_bytes().to_vec()]
        .concat()
}

fn beacon(ies: &[u8]) -> Vec<u8> {
    [&[0x80, 0, 0, 0][..], &[0xFF; 6], &AP, &AP, &[0, 0], &[0; 12], ies].concat()
}

#[test]
fn the_aid_comes_only_from_the_access_points_own_success() {
    assert_eq!(granted_aid(&assoc_resp(AP, 0, 7), AP), Some(7), "the AID, its top bits off");
    assert_eq!(granted_aid(&assoc_resp(AP, 17, 7), AP), None, "a refusal grants none");
    assert_eq!(granted_aid(&assoc_resp([0x02; 6], 0, 7), AP), None, "another sender's");
    assert_eq!(granted_aid(&assoc_resp(AP, 0, 7)[..28], AP), None, "a short frame");
}

#[test]
fn the_dtim_period_is_the_tims_and_one_without_it() {
    assert_eq!(dtim_period(&beacon(&[0, 3, b'n', b'e', b't', 5, 4, 0, 3, 0, 0])), 3);
    assert_eq!(dtim_period(&beacon(&[0, 3, b'n', b'e', b't'])), 1, "no TIM");
    assert_eq!(dtim_period(&beacon(&[5, 4, 0, 0, 0, 0])), 1, "a period of zero");
    assert_eq!(dtim_period(&beacon(&[5, 1, 0])), 1, "a TIM too short to name one");
    assert_eq!(dtim_period(&[0x80; 10]), 1, "no room for elements");
}
