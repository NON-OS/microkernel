// NONOS Operating System (AGPL-3.0-or-later)

//! The version 1 context info as the ROM reads it after a boot, field by
//! field at `struct iwl_context_info`'s offsets, every image map entry
//! pointing at its section's bytes.

use super::rig::{polls, rig, run, QU_B, QU_B0};
use crate::gen3::dram_map::classify;
use crate::gen3::plan::{CMD_TFDS, CTXT_INFO, RB_STTS, RX_FREE, RX_USED};
use crate::gen3::region::Region;

fn le(b: &[u8], at: usize, n: usize) -> u64 {
    b[at..at + n].iter().rev().fold(0, |a, &x| a << 8 | u64::from(x))
}

#[test]
fn the_context_info_names_the_rings_and_every_section() {
    let r = rig(QU_B, QU_B0, false);
    run(&r, &mut polls(), |_, _, _| ()).0.expect("alive");
    let mut ci = vec![0u8; 1792];
    r.model.ctrl.read(CTXT_INFO, &mut ci);
    let base = r.model.ctrl.dev;
    assert_eq!(
        (le(&ci, 0, 2), le(&ci, 2, 2), le(&ci, 4, 2)),
        (0x331, 0, 448),
        "mac id, version, dwords"
    );
    assert_eq!(le(&ci, 8, 4), 0x100 | 9 << 4 | 2 << 9, "long TFDs, 512 RBDs, 2 KiB buffers");
    assert_eq!(le(&ci, 24, 8), base + RX_FREE as u64);
    assert_eq!(le(&ci, 32, 8), base + RX_USED as u64);
    assert_eq!(le(&ci, 40, 8), base + RB_STTS as u64);
    assert_eq!((le(&ci, 48, 8), ci[56]), (base + CMD_TFDS as u64, 2), "32 command TFDs");
    assert!(ci[57..192].iter().all(|&b| b == 0), "the rest of the head left zero");
    let fw = classify(QU_B);
    for (k, secs) in [(0, &fw.umac), (1, &fw.lmac), (2, &fw.virt)] {
        for i in 0..64 {
            let addr = le(&ci, 192 + (k * 64 + i) * 8, 8);
            let Some(s) = secs.get(i) else {
                assert_eq!(addr, 0, "map {k} entry {i} unused");
                continue;
            };
            let region =
                r.fw.iter().find(|m| addr >= m.dev && addr < m.dev + m.len() as u64).unwrap();
            let mut got = vec![0u8; s.data.len()];
            region.read((addr - region.dev) as usize, &mut got);
            assert!(got == s.data, "map {k} entry {i}");
        }
    }
}
