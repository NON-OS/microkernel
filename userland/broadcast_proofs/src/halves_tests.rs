// NONOS Operating System (AGPL-3.0-or-later)
//! The count read by halves lies between the system count at the first
//! read and at the last, whatever carry falls between the three reads.

use crate::halves::combine;

#[test]
fn a_count_by_halves_lies_between_the_first_read_and_the_last() {
    let carries = [0u64, 1 << 32, 0xffff_ffff, 0x1_ffff_fff0, 0xffff_fffe_ffff_ff00];
    for base in carries {
        for start in base.saturating_sub(40)..base + 40 {
            for (gap1, gap2) in [(0u64, 0u64), (1, 0), (0, 1), (3, 7), (40, 40), (1 << 33, 1)] {
                let (t0, t1) = (start, start.saturating_add(gap1));
                let t2 = t1.saturating_add(gap2);
                let got = combine((t0 >> 32) as u32, t1 as u32, (t2 >> 32) as u32);
                assert!(t0 <= got && got <= t2, "{t0:#x} {t1:#x} {t2:#x} gave {got:#x}");
                if t0 >> 32 == t2 >> 32 {
                    assert_eq!(got, t1, "unchanged high halves read one count");
                }
            }
        }
    }
}
