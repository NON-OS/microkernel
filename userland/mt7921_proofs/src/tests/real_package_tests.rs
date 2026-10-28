// NONOS Operating System (AGPL-3.0-or-later)

//! The real package, when a run names a directory holding it
//! (`NONOS_MT7921_PACKAGE`, e.g. the ESP's EFI/nonos/firmware/mediatek that
//! `scripts/firmware_package.py` fills outside the tree): the MT7922's pinned
//! files pass their pins, parse as the layouts say, and bring the modelled
//! chip to Ready, the patch one AES-keyed section at 0x900000, the RAM code
//! five regions with the CLC region left out, started at its override
//! address. Without the variable there is nothing to read and nothing is
//! claimed; no MediaTek byte is ever in the tree.

use std::path::Path;

use nonos_libc::set_slot;

use crate::bringup::run_with;
use crate::fw::package::{for_chip, FILES};
use crate::model::rig::{chip, mapped, regs};
use crate::stage::Stage;

#[test]
fn the_pinned_mt7922_files_bring_the_model_up() {
    let Some(dir) = std::env::var_os("NONOS_MT7921_PACKAGE") else { return };
    for f in &FILES {
        set_slot(f.kind, std::fs::read(Path::new(&dir).join(f.name)).ok());
    }
    let c = chip();
    let (radio, stage) = run_with(regs(&c), &mapped(), for_chip);
    assert!(radio.is_some() && stage == Stage::Ready, "{stage:?}");
    let k = c.borrow();
    let pieces: Vec<(u8, u32, u32)> =
        k.mcu.pieces.iter().map(|p| (p.cmd, p.addr, p.mode)).collect();
    assert_eq!(pieces[0], (0x05, 0x900000, 1 << 31 | 1 << 3 | 1), "the patch: AES, key 0");
    let ram: Vec<u32> = pieces[1..].iter().map(|p| p.1).collect();
    assert_eq!(ram, [0x91b000, 0x2015c00, 0x404400, 0x210a000, 0xe0200000], "no CLC region");
    assert_eq!(k.mcu.started, Some((1, 0x91b000)), "FW_START_OVERRIDE at the first region");
    assert!(k.mcu.pieces.iter().all(|p| p.bytes.len() == p.len as usize), "every byte sent");
}
