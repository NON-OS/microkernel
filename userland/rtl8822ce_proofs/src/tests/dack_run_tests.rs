// NONOS Operating System (AGPL-3.0-or-later)
//! The DAC calibration on the modelled chip: at most ten ADC rounds and ten
//! DAC rounds a path, each stopping once the DC left is under 5; every
//! register it saved put back after, RF 0x8F of both paths from rtw88's one
//! slot (path B's); a result that does not read back reported, not fatal.

use crate::model::Chip;
use crate::phy::dack::run;
use crate::phy::rf::set_paths;

/// `DACK_REG_8822C`'s registers, as rtw8822c_dac_backup_reg lists them.
#[rustfmt::skip]
const SAVED: [usize; 16] = [0x180C, 0x1810, 0x410C, 0x4110, 0x1C3C, 0x1C24, 0x1D70, 0x09B4,
    0x1A00, 0x1A14, 0x1D58, 0x1C38, 0x1E24, 0x1E28, 0x1860, 0x4160];

fn calibrated(sample: (u32, u32)) -> (Chip, bool) {
    let c = Chip::new();
    let word = sample.0 << 12 | sample.1;
    *c.hw.sample.borrow_mut() = Some(Box::new(move || word));
    let ok = run(&c);
    (c, ok)
}

/// The ADC and DAC rounds run on the path at `base`: the ADC writes its path's
/// select to 0x1C3C once a round, the DAC steps 1 and 3 its base + 0xB0.
fn rounds(c: &Chip, base: usize) -> (usize, usize) {
    let sel = if base == 0x1800 { 0xA_8003 } else { 0x8_8003 };
    let adc = c.at(0x1C3C).into_iter().filter(|&v| v == sel).count();
    let dac = c.at(base + 0xB0).into_iter().filter(|&v| v == 0x0A11_FB88).count() / 2;
    (adc, dac)
}

#[test]
fn a_dc_that_stays_ends_at_ten_rounds() {
    let _one = super::lock();
    set_paths(2);
    let (c, ok) = calibrated((0x10, 0x10)); // the ADC never settles
    assert_eq!(rounds(&c, 0x1800).0, 10);
    assert_eq!(rounds(&c, 0x4100).0, 10);
    let (c, _) = calibrated((0, 0)); // what step 3 leaves never settles
    assert_eq!(rounds(&c, 0x1800), (1, 10));
    assert_eq!(rounds(&c, 0x4100), (1, 10));
    assert!(!ok, "codes that never read back are reported");
}

#[test]
fn a_dc_under_five_ends_the_rounds() {
    let _one = super::lock();
    set_paths(2);
    let (c, _) = calibrated((0x10, 0x10));
    assert_eq!(rounds(&c, 0x1800).1, 1, "0x10 is step 3's zero");
    let (c, _) = calibrated((0x0, 0x0));
    assert_eq!(rounds(&c, 0x4100).0, 1, "0 is the ADC's zero");
}

#[test]
fn what_it_saved_is_put_back() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    *c.hw.sample.borrow_mut() = Some(Box::new(|| 0));
    for (i, r) in SAVED.iter().enumerate() {
        c.set32(*r, 0x5A00_0000 + i as u32);
    }
    c.set32(0x3C00 + (0x8F << 2), 0x111);
    c.set32(0x4C00 + (0x8F << 2), 0x222);
    run(&c);
    for (i, r) in SAVED.iter().enumerate() {
        assert_eq!(c.get32(*r), 0x5A00_0000 + i as u32, "{r:#x}");
    }
    assert_eq!(c.get32(0x3C00 + (0x8F << 2)) & 0xF_FFFF, 0x222, "path B's slot");
    assert_eq!(c.get32(0x4C00 + (0x8F << 2)) & 0xF_FFFF, 0x222);
    assert_eq!(c.get32(0x4130) & 1 << 30, 1 << 30);
}
