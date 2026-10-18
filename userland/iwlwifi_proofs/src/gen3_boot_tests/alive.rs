// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware boots to ALIVE, and a SKU part rings the platform-NVM doorbell.

use super::rig::{polls, rig, run_boot, SO_GF};
use crate::gen3::dram_map::classify;
use crate::gen3::region::Region;

#[test]
fn the_firmware_boots_to_alive_with_its_memory_where_the_context_info_says() {
    let r = rig();
    let mut c = polls();
    let alive = run_boot(&r, &mut c).expect("alive");
    assert!(alive.ok());
    // The context info names the control region's structures by address.
    let ctrl = &r.model.ctrl;
    let mut ci = [0u8; 104];
    ctrl.read(0, &mut ci);
    let rd64 = |o: usize| u64::from_le_bytes(ci[o..o + 8].try_into().unwrap());
    assert_eq!(rd64(8), ctrl.dev() + 0x2000, "peripheral info page");
    assert_eq!(rd64(88), ctrl.dev() + 0x1000, "peripheral scratch");
    assert_eq!(u32::from_le_bytes(ci[96..100].try_into().unwrap()), 1660, "scratch size without FSEQ");
    assert_eq!(u16::from_le_bytes([ci[68], ci[69]]), 4, "128-entry command ring");
    assert_eq!(u16::from_le_bytes([ci[70], ci[71]]), 9, "512-entry completion ring");
    // The kick went to the control region; the CPU was started through the
    // UMAC window with the 24-bit address intact.
    assert_eq!(r.model.reg(0x118) as u64 | (r.model.reg(0x11C) as u64) << 32, ctrl.dev());
    assert!(r.model.s.borrow().prph.contains(&(0xD0_5C44, 1)), "UREG_CPU_INIT_RUN at its UMAC address");
    // Every section is in its block: the first LMAC section's bytes are there.
    let layout = classify(SO_GF);
    let first = layout.lmac[0].data;
    assert_eq!(&r.fw[0].bytes.borrow()[..first.len()], first);
}

#[test]
fn a_part_with_a_sku_id_rings_the_platform_nvm_doorbell_and_waits() {
    let r = rig();
    r.model.s.borrow_mut().alive_sku = [1, 2, 3];
    let mut c = polls();
    let alive = run_boot(&r, &mut c).expect("alive and the PNVM step done");
    assert_eq!(alive.sku_id, [1, 2, 3]);
    assert!(r.model.s.borrow().prph.contains(&(0xD0_5C04, 1 << 20)), "the PNVM doorbell");
}
