// NONOS Operating System (AGPL-3.0-or-later)
//! INTID decode: what is finished, and where each kind's handler lives.

use crate::gic::intid::{acknowledged, is_lpi, is_spi, slot, LPI_BASE, LPI_COUNT, SLOTS};

#[test]
fn only_the_special_values_skip_the_end_of_interrupt() {
    for special in 1020..=1023 {
        assert_eq!(acknowledged(special), None);
    }
    for activated in [0, 15, 31, 32, 1019, LPI_BASE, LPI_BASE + LPI_COUNT, 0xff_ffff] {
        assert_eq!(acknowledged(activated), Some(activated), "{activated}");
    }
}

#[test]
fn spis_and_lpis_are_told_apart() {
    assert!(!is_spi(31) && is_spi(32) && is_spi(1019) && !is_spi(1020) && !is_spi(LPI_BASE));
    assert!(!is_lpi(LPI_BASE - 1) && is_lpi(LPI_BASE) && is_lpi(LPI_BASE + LPI_COUNT - 1));
    assert!(!is_lpi(LPI_BASE + LPI_COUNT) && !is_lpi(1019));
}

#[test]
fn every_handled_intid_has_its_own_slot_and_no_other_has_one() {
    let mut seen = vec![false; SLOTS];
    let handled = (0..1020).chain(LPI_BASE..LPI_BASE + LPI_COUNT);
    for intid in handled {
        let at = slot(intid).unwrap();
        assert!(!seen[at], "{intid} shares slot {at}");
        seen[at] = true;
    }
    assert!(seen.iter().all(|s| *s));
    for none in [1020, 1023, 1024, LPI_BASE - 1, LPI_BASE + LPI_COUNT, u32::MAX] {
        assert_eq!(slot(none), None, "{none}");
    }
}
