// NONOS Operating System (AGPL-3.0-or-later)

//! The DAC calibration's conversions of a measured DC offset (a 10-bit
//! two's-complement value; 0x200 and above negative), each as rtw88 writes
//! it in rtw8822c.c. Pure, so each is held on the host.

/// `rtw8822c_dac_cal_adc`'s compensation: 0x400 less the offset, unless zero.
pub fn compensation(v: u32) -> u32 {
    if v != 0 {
        0x400 - v
    } else {
        v
    }
}

/// An offset's distance from zero (the `>= 0x200` fold rtw88 does before
/// comparing with 5).
pub fn magnitude(v: u32) -> u32 {
    if v >= 0x200 {
        0x400 - v
    } else {
        v
    }
}

/// `rtw8822c_dac_cal_step2`: the compensation as the code step 3 writes, a
/// 6/5 slope about 0x80 below 0x300 and from 0x7F above (wrapping as rtw88's
/// unsigned arithmetic wraps).
pub fn code(v: u32) -> u32 {
    let c = compensation(v);
    if c < 0x300 {
        c * 2 * 6 / 5 + 0x80
    } else {
        0x7F_u32.wrapping_sub((0x400 - c) * 2 * 6 / 5)
    }
}

/// `rtw8822c_dac_cal_step3`: a DC measured with the 0x10 bias the ADC
/// reference carries, unbiased and folded to its distance from zero.
pub fn left(v: u32) -> u32 {
    magnitude(if v >= 0x10 { v - 0x10 } else { 0x400 - (0x10 - v) })
}
