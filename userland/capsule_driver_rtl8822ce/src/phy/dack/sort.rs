// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_dac_iq_sort` (rtw8822c.c:185-208): both sample vectors bubble
//! sorted, ascending with the negatives (0x200 and above) first, so the
//! offset is taken over the middle of each.

use super::iq::SN;

/// `__rtw8822c_dac_iq_sort`: whether `a` and `b` swap.
pub fn out_of_order(a: u32, b: u32) -> bool {
    match (a >= 0x200, b >= 0x200) {
        (true, true) | (false, false) => a > b,
        (false, true) => true,
        (true, false) => false,
    }
}

pub fn sort(iv: &mut [u32; SN], qv: &mut [u32; SN]) {
    for i in 0..SN - 1 {
        for j in 0..SN - 1 - i {
            for v in [&mut *iv, &mut *qv] {
                if out_of_order(v[j], v[j + 1]) {
                    v.swap(j, j + 1);
                }
            }
        }
    }
}
