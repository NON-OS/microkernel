// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_rfk_parser` (Linux v6.12 rtw89 `phy.c`) over the calibration
//! lists `tables/gen_rfk.py` writes to `rfk.bin`: RF writes, BB mask
//! writes, BB set and clear, and delays (in microseconds; the clock's least
//! step, 1 ms, stands in, and only lengthens a settle).

use crate::phy::bb::Bb;
use crate::regs::Mmio;
use crate::wait::{delay_ms, ms_of};

pub const RFK: &[u8] = include_bytes!("../../tables/rfk.bin");

/// The lists, in `gen_rfk.py`'s order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum List {
    AfeInit,
    CheckAddcA,
    CheckAddcB,
    CheckDadcEnA,
    CheckDadcEnB,
    CheckDadcDisA,
    CheckDadcDisB,
    DackS0_1,
    DackS0_2,
    DackS0_3,
    DackS1_1,
    DackS1_2,
    DackS1_3,
    IqkSetNonDbcc,
    IqkRestoreNonDbcc,
}
pub const LISTS: usize = 15;

fn word(i: usize) -> u32 {
    u32::from_le_bytes([RFK[4 * i], RFK[4 * i + 1], RFK[4 * i + 2], RFK[4 * i + 3]])
}

/// The entries of `list`: (flag, path, addr, mask, data).
pub fn entries(list: List) -> impl Iterator<Item = [u32; 5]> {
    let n = list as usize;
    let start = LISTS + (0..n).map(|i| word(i) as usize * 5).sum::<usize>();
    (0..word(n) as usize).map(move |e| core::array::from_fn(|k| word(start + 5 * e + k)))
}

pub fn run(bb: &Bb<'_, impl Mmio>, list: List) {
    for [flag, path, addr, mask, data] in entries(list) {
        match flag {
            0 => {
                bb.write_rf(path as u8, addr, mask, data);
            }
            1 => bb.mask(addr, mask, data),
            2 => bb.set(addr, mask),
            3 => bb.clr(addr, mask),
            _ => delay_ms(ms_of(data as u64)),
        }
    }
}
