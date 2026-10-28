// NONOS Operating System (AGPL-3.0-or-later)

//! The 8822C's DAC calibration (DACK), which rtw88 runs in the driver at
//! every PHY bring-up (`rtw8822c_rf_dac_cal`, rtw8822c.c:943): the registers
//! it touches saved, then per path the ADC DC (`adc`), up to ten rounds of
//! the DAC's own calibration and DC correction (`step`, `step3`) until the
//! DC left is under 5, and the path closed; then everything restored.
//! rtw88 first tries to restore an earlier run's results; on the first
//! bring-up of a boot there are none, so it calibrates, and so does this.
//! Its save of the results for that restore is not kept: a boot runs this
//! once.

pub mod adc;
pub mod codes;
pub mod iq;
pub mod save;
pub mod search;
pub mod sort;
pub mod step;
pub mod step3;

use crate::phy::rf::{PATH_A, PATH_B};
use crate::regs::Mmio;

/// The rounds rtw88 gives the ADC and the DAC calibration of a path.
pub const ADCK_ROUNDS: usize = 10;
pub const DACK_ROUNDS: usize = 10;

/// Where a path's calibration registers are written, and read back.
pub fn write_base(path: usize) -> usize {
    if path == PATH_A {
        0x1800
    } else {
        0x4100
    }
}

pub fn read_base(path: usize) -> usize {
    if path == PATH_A {
        0x2800
    } else {
        0x4500
    }
}

/// Calibrate both paths. False when a step's results did not read back,
/// which rtw88 only logs; the calibration still runs to its end.
pub fn run<M: Mmio>(mmio: &M) -> bool {
    let saved = save::save(mmio);
    save::bb_setting(mmio);
    let mut ok = true;
    for path in [PATH_A, PATH_B] {
        let a = adc::adc(mmio, path);
        for _ in 0..DACK_ROUNDS {
            ok &= step::step1(mmio, path, a.word);
            let code = step::step2(mmio, path);
            let left = step3::step3(mmio, path, (a.ic, a.qc), code);
            ok &= left.took;
            if left.ic < 5 && left.qc < 5 {
                break;
            }
        }
        step3::step4(mmio, path);
    }
    mmio.write32(0x1B00, 0x0000_0008);
    crate::phy::bits::write32_mask(mmio, 0x4130, 1 << 30, 0x1);
    mmio.write8(0x1BCC, 0x0);
    mmio.write32(0x1B00, 0x0000_000A);
    mmio.write8(0x1BCC, 0x0);
    save::restore(mmio, &saved);
    ok
}
