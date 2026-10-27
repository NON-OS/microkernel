// NONOS Operating System (AGPL-3.0-or-later)

//! The download on the model, against `mt76_connac2_load_patch` and
//! `mt76_connac_mcu_send_ram_firmware`: every piece announced with its
//! address, length and mode and its bytes delivered exactly; the patch
//! skipped when the chip has it; a patch out of shape refused before the
//! semaphore; the CLC region never sent; the start at the override.

use super::mcu_tests::card;
use crate::fw::{download, Step};
use crate::model::images::{patch, patch_7922, pattern, ram_7922};

pub fn run(c: &mut crate::card::Card, p: &[u8], r: &[u8]) -> Result<(), Step> {
    download(&mut c.mcu, &c.regs, &mut c.dma, p, r)
}

#[test]
fn every_piece_arrives_whole_with_its_announcement() {
    let (mut k, c) = card();
    run(&mut k, &patch_7922(), &ram_7922()).unwrap();
    let m = &c.borrow().mcu;
    let got: Vec<(u8, u32, u32, u32)> =
        m.pieces.iter().map(|p| (p.cmd, p.addr, p.len, p.mode)).collect();
    let enc = 1 << 31 | 0x9;
    assert_eq!(
        got,
        vec![
            (0x05, 0x900000, 9000, enc),
            (0x01, 0x91b000, 5000, enc),
            (0x01, 0x2015c00, 4096, enc),
            (0x01, 0x404400, 100, enc)
        ],
        "the CLC region was not sent"
    );
    assert_eq!(m.pieces[0].bytes, pattern(1, 9000));
    assert_eq!(m.pieces[1].bytes, pattern(2, 5000));
    assert_eq!(m.started, Some((1, 0x91b000)), "FW_START_OVERRIDE at region 0");
    assert!(!m.sem_held);
}

#[test]
fn a_chip_that_has_its_patch_is_not_sent_it_again() {
    let (mut k, c) = card();
    c.borrow_mut().mcu.patched = true;
    run(&mut k, &patch_7922(), &ram_7922()).unwrap();
    assert!(c.borrow().mcu.pieces.iter().all(|p| p.addr != 0x900000));
}

#[test]
fn a_patch_out_of_shape_is_refused_before_the_semaphore() {
    let (mut k, c) = card();
    let mut bad = patch(&[(0x900000, pattern(1, 10), 0)]);
    bad[96 + 3] = 9;
    assert_eq!(run(&mut k, &bad, &ram_7922()), Err(Step::BadPatch));
    assert!(c.borrow().mcu.seen.is_empty());
}
