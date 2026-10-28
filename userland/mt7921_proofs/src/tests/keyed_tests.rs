// NONOS Operating System (AGPL-3.0-or-later)

//! A frame for the chip to seal is plaintext marked Protected: it goes out
//! only to the access point and only while the access point's entry holds
//! the pairwise key, which the firmware must have taken and a new
//! association's reset of the entry takes away. Otherwise it is refused,
//! and nothing goes on the air in the clear. A key needs its entry: none
//! is sent before the association that makes it. A key command is wiped,
//! its body and its ring slot, once the firmware answered it.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{ChipKey, Radio};

use crate::mcu::cmd::STA_REC_UPDATE;
use crate::model::net::{assoc, AP};
use crate::model::rig::up;
use crate::vif::key::pairwise;

/// Data to the access point, Protected, not yet sealed.
fn for_chip(sa: [u8; 6]) -> Vec<u8> {
    let mut f = vec![0x08, 0x41, 0, 0];
    f.extend_from_slice(&AP);
    f.extend_from_slice(&sa);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0, 0, 0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45]);
    f
}

#[test]
fn a_frame_for_the_chip_goes_out_only_under_a_key_it_took() {
    let (mut r, c) = up();
    let f = for_chip(r.mac());
    let tk = ChipKey::Pairwise(&[1; 16]);
    r.tune(Band::Ghz2, 6);
    assert!(!r.set_key(&tk), "no entry for the key before the association");
    assert!(!r.send_keyed(&f), "no access point");
    r.associated(Some(&assoc()));
    assert!(!r.send_keyed(&f), "its entry holds no key");
    c.borrow_mut().faults.refuse_keys = true;
    assert!(!r.set_key(&tk));
    assert!(!r.send_keyed(&f), "the firmware refused the key");
    c.borrow_mut().faults.refuse_keys = false;
    assert!(r.set_key(&tk) && r.send_keyed(&f));
    r.associated(Some(&assoc()));
    assert!(!r.send_keyed(&f), "a new association reset the entry and its key");
    let chip = c.borrow();
    let sealed: Vec<_> = chip.sent.iter().filter(|t| t.txd[3] & 2 != 0).collect();
    assert_eq!(sealed.len(), 1, "one frame went for the chip to seal");
    assert!(sealed[0].air.as_ref().is_some_and(|a| a.len() == f.len() + 16));
}

#[test]
fn a_key_command_is_wiped_once_answered() {
    let (mut k, _c) = super::mcu_tests::card();
    let mut body = pairwise(&[7; 16]);
    assert!(k.secret(STA_REC_UPDATE, &mut body), "the firmware took it");
    assert!(body.iter().all(|&b| b == 0), "the body");
    assert!(!nonos_libc::dma_holds(&[7; 16]), "the ring slot");
}
