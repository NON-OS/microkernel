// NONOS Operating System (AGPL-3.0-or-later)
//! What a read returns where the chip, not the last write, decides: the
//! status bits rtw89 polls on, set the way the hardware sets them once the
//! step that raises them has run. A ring index read first lets the card
//! fill what it can, as its DMA engine runs on its own.

use super::rxq;
use super::state::State;

const BB: u32 = 0x1_0000;
/// DACK done flags, path 0 then 1 (`_dack_s0_check_done`, `_s1_`).
const DACK_DONE: [(u32, u32); 8] = [
    (BB + 0xc040, 1 << 31),
    (BB + 0xc064, 1 << 31),
    (BB + 0xc05c, 0x4),
    (BB + 0xc080, 0x4),
    (BB + 0xc140, 1 << 31),
    (BB + 0xc164, 1 << 31),
    (BB + 0xc15c, 0x4),
    (BB + 0xc180, 0x4),
];

fn computed(s: &mut State, a: u32, v: u32) -> u32 {
    match a {
        0x4 => v | 0x2_0000, // R_AX_SYS_PW_CTRL: B_AX_RDY_SYSPWR
        0x3f0 => (v & !0x300) | if s.powered { 0x100 } else { 0 }, // IC_PWR_STATE
        // WDE and PLE ready once the DLE is enabled (R_AX_DMAC_FUNC_EN).
        0x8d00 | 0x9100 if s.word(0x8400) & 0x0480_0000 == 0x0480_0000 => v | 0x3,
        // HCI flow control: no public page in use, every one available.
        0x8a9c => (v & !0x1fff) | (s.word(0x8a94) & 0x1fff),
        0x9e10 => v | 1 << 31,        // R_AX_SS_CTRL: B_AX_SS_INIT_DONE_1
        0xdaf0 => v | 1 << 29,        // R_AX_LTE_CTRL: ready
        0x1c0d0 => v | 0x8,           // R_DRCK_RS: done
        0x1c0fc | 0x1c1fc => v | 0x1, // R_ADDCKR0/1: done
        0x1050 | 0x1054 => rxq::fill(s, a, v),
        _ => DACK_DONE.iter().filter(|d| d.0 == a).fold(v, |v, d| v | d.1),
    }
}

/// The word at `a` as the driver reads it.
pub fn read(s: &mut State, a: u32) -> u32 {
    let v = s.word(a);
    let v = computed(s, a, v);
    s.faults.apply(a, v)
}
