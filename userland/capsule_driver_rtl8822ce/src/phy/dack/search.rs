// NONOS Operating System (AGPL-3.0-or-later)

//! One DC measurement (rtw88 `rtw8822c_dac_cal_rf_mode`, rtw8822c.c:287-376):
//! 100 samples within range out of at most 10000 reads of the sample
//! register, then up to 101 rounds that sort them and re-read the two ends
//! while the spread is over 5, and the means of the middle 80. Every loop
//! is bounded by a count, as rtw88 bounds it; none waits on the chip.

use super::iq::{check, delta, minmax, offset, SN};
use super::sort::sort;
use crate::phy::bits::read32_mask;
use crate::regs::Mmio;

/// The I/Q sample register: I in bits 21:12, Q in 9:0.
const REG_SAMPLE: usize = 0x2DBC;
/// rtw88's bounds: reads to fill the samples, and further search rounds.
pub const SAMPLE_READS: u32 = 10_000;
pub const SEARCH_ROUNDS: u32 = 100;

/// One (I, Q) sample read from the chip.
pub fn read_sample<M: Mmio>(mmio: &M) -> (u32, u32) {
    let t = read32_mask(mmio, REG_SAMPLE, 0x3F_FFFF);
    ((t & 0x3F_F000) >> 12, t & 0x3FF)
}

/// The I and Q DC offsets from samples `read` gives.
pub fn measure(mut read: impl FnMut() -> (u32, u32)) -> (u32, u32) {
    let (mut iv, mut qv) = ([0u32; SN], [0u32; SN]);
    let (mut i, mut reads) = (0, 0);
    while i < SN && reads < SAMPLE_READS {
        reads += 1;
        (iv[i], qv[i]) = read();
        if check(iv[i]) && check(qv[i]) {
            i += 1;
        }
    }
    let mut round = 0;
    loop {
        let (mut i_min, mut i_max, mut q_min, mut q_max) = (iv[0], iv[0], qv[0], qv[0]);
        for k in 0..SN {
            minmax(iv[k], &mut i_min, &mut i_max);
            minmax(qv[k], &mut q_min, &mut q_max);
        }
        sort(&mut iv, &mut qv);
        if delta(i_min, i_max) <= 5 && delta(q_min, q_max) <= 5 {
            break;
        }
        (iv[0], qv[0]) = read();
        (iv[SN - 1], qv[SN - 1]) = read();
        if round >= SEARCH_ROUNDS {
            break;
        }
        round += 1;
    }
    (offset(&iv), offset(&qv))
}
