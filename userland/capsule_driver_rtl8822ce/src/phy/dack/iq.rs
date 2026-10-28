// NONOS Operating System (AGPL-3.0-or-later)

//! The arithmetic of rtw88's 8822C DAC calibration on its I/Q samples
//! (rtw8822c.c:156-285). A sample is a 10-bit two's-complement value: 0x200
//! and above are negative. Pure, so each rule is held on the host.

/// `DACK_SN_8822C`: samples per measurement.
pub const SN: usize = 100;

/// `rtw8822c_dac_iq_check`: a sample within 0x64 of zero either way.
pub fn check(v: u32) -> bool {
    !((v >= 0x200 && 0x400u32.wrapping_sub(v) > 0x64) || (v < 0x200 && v > 0x64))
}

/// `rtw8822c_rf_minmax_cmp`: `min` and `max` moved by `v`, signed.
pub fn minmax(v: u32, min: &mut u32, max: &mut u32) {
    if v >= 0x200 {
        if *min < 0x200 || *min > v {
            *min = v;
        }
        if *max >= 0x200 && *max < v {
            *max = v;
        }
    } else {
        if *min < 0x200 && *min > v {
            *min = v;
        }
        if *max >= 0x200 || *max < v {
            *max = v;
        }
    }
}

/// The spread between `min` and `max` (rtw8822c_dac_cal_iq_search).
pub fn delta(min: u32, max: u32) -> u32 {
    if (max < 0x200) == (min < 0x200) {
        max.wrapping_sub(min)
    } else {
        max.wrapping_add(0x400u32.wrapping_sub(min))
    }
}

/// `rtw8822c_dac_iq_offset`: the mean of the middle 80 samples, as a 10-bit
/// two's-complement value.
pub fn offset(vec: &[u32; SN]) -> u32 {
    let (mut p, mut m) = (0u32, 0u32);
    for &v in &vec[10..SN - 10] {
        if v > 0x200 {
            m += 0x400 - v;
        } else {
            p += v;
        }
    }
    let n = (SN - 20) as u32;
    if p > m {
        (p - m) / n
    } else {
        let t = (m - p) / n;
        if t != 0 {
            0x400 - t
        } else {
            0
        }
    }
}
