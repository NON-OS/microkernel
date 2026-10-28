// NONOS Operating System (AGPL-3.0-or-later)
//! The signal from the 8822C's PHY status: page 0 (CCK) is path A's PWDB
//! less 110, two dB a step back for a gain index outside the AGC bounds;
//! page 1 (OFDM) is the stronger path's; never below -120; other pages none.

use crate::rx::signal::{set_cck_bounds, signal_dbm};

fn page0(pwdb: u32, gain: u32) -> [u32; 4] {
    [gain << 16 | pwdb << 8, 0, 0, 0]
}

#[test]
fn a_cck_frame_is_path_as_power_corrected_for_its_gain() {
    let _one = super::lock();
    set_cck_bounds(0x30, 0x10);
    assert_eq!(signal_dbm(page0(60, 0x20), 0), Some(-50), "inside the bounds");
    assert_eq!(signal_dbm(page0(60, 0x0E), 0), Some(-46), "two below: +4 dB");
    assert_eq!(signal_dbm(page0(60, 0x33), 0), Some(-56), "three above: -6 dB");
    set_cck_bounds(0x3F, 0);
    assert_eq!(signal_dbm(page0(60, 0x0E), 0), Some(-50), "no bounds, no correction");
}

#[test]
fn an_ofdm_frame_is_the_stronger_paths() {
    let w = |a: u32, b: u32| [1 | a << 8 | b << 16, 0, 0, 0];
    assert_eq!(signal_dbm(w(50, 70), 0), Some(-40));
    assert_eq!(signal_dbm(w(70, 50), 0), Some(-40));
}

#[test]
fn a_reading_is_floored_and_other_pages_are_none() {
    let _one = super::lock();
    set_cck_bounds(0x3F, 0);
    assert_eq!(signal_dbm(page0(0, 0x20), 0), Some(-110));
    assert_eq!(signal_dbm([1, 0, 0, 0], 0), Some(-110));
    set_cck_bounds(0, 0);
    assert_eq!(signal_dbm(page0(0, 0x3F), 0), Some(-120), "-236 floored at -120");
    for page in 2..16 {
        assert_eq!(signal_dbm([page, 0, 0, 0], 0), None);
    }
}
