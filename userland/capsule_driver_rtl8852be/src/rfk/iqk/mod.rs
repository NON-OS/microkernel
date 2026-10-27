// NONOS Operating System (AGPL-3.0-or-later)

//! TX/RX IQ imbalance calibration, `_doiqk` per path (Linux v6.12 rtw89
//! `rtw8852b_rfk.c`): note the channel, back up the BB and RF registers
//! IQK disturbs, set up the BB, calibrate (LOK up to three tries, TX, RX),
//! load the results, restore. The caller pauses TX and waits for RX mode.

pub mod groups;
pub mod lok;
pub mod save;
pub mod setup;
pub mod shot;
pub mod txk;
pub mod txset;

use crate::chan::Chan;
use crate::phy::bb::Bb;
use crate::regs::Mmio;

/// `struct rtw89_iqk_info`'s part that outlives one run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Iqk {
    pub times: u8,
    pub fail_cnt: u8,
    pub table_idx: [u8; 2],
    pub mcc_ch: [[u8; 2]; 2],
    /// Per path: LOK coarse, LOK fine, TX, RX failed.
    pub fail: [[bool; 4]; 2],
}

/// `_iqk_init` past its first run only clears the info word.
pub fn init(bb: &Bb<'_, impl Mmio>) {
    bb.mask(groups::R_IQKINF, u32::MAX, 0);
}

/// `_iqk_by_path`: the four outcomes.
fn by_path(bb: &Bb<'_, impl Mmio>, path: u8, five: bool) -> [bool; 4] {
    txset::clk(bb, [0x1f, 0x13], [0x0001, 0x0041]);
    let mut l = [false; 3];
    for ibias in 1..=3 {
        txset::res_table(bb, path, five, ibias);
        txset::txk_setting(bb, path, five);
        l = lok::lok(bb, path, five);
        if !l[2] {
            break;
        }
    }
    let tx = txk::txk(bb, path, five);
    groups::rx_setup(bb, path, five);
    let rx = groups::rxk(bb, path, five);
    [l[0], l[1], tx, rx]
}

pub fn doiqk(r: &impl Mmio, s: &mut Iqk, path: u8, c: Chan) {
    let bb = Bb(r);
    s.times = s.times.wrapping_add(1);
    setup::ch_info(&bb, s, path, c);
    let saved = save::backup(&bb, path);
    save::macbb(&bb);
    setup::preset(&bb, s, path);
    let f = by_path(&bb, path, c.five);
    s.fail[path as usize] = f;
    setup::info(&bb, s, path);
    let cfir = |failed| if failed { 0x4000_0002 } else { 0x4000_0000 };
    save::restore(&bb, path, cfir(f[2]), cfir(f[3]));
    save::afebb_restore(&bb);
    save::restore_backup(&bb, path, &saved);
}

/// `_iqk` for both paths.
pub fn iqk(r: &impl Mmio, s: &mut Iqk, c: Chan) {
    init(&Bb(r));
    doiqk(r, s, 0, c);
    doiqk(r, s, 1, c);
}
