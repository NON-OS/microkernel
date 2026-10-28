// NONOS Operating System (AGPL-3.0-or-later)

//! The DAC calibration's last two steps on one path (rtw88
//! `rtw8822c_dac_cal_step3` and `_step4`, rtw8822c.c:566-651): step 2's
//! codes written and run, the DC that is left measured against the ADC
//! reference; then the path's calibration mode closed.

use super::codes::left;
use super::search::{measure, read_sample};
use super::{read_base, write_base};
use crate::phy::bits::{ready, write32_mask};
use crate::regs::Mmio;
use crate::wait::delay_ms;

/// What step 3 leaves: the DC left (I, Q) as a distance from zero, which
/// ends the rounds under 5, and whether the codes read back as written.
pub struct Left {
    pub ic: u32,
    pub qc: u32,
    pub took: bool,
}

pub fn step3<M: Mmio>(mmio: &M, path: usize, adc: (u32, u32), code: (u32, u32)) -> Left {
    let (base, read) = (write_base(path), read_base(path));
    let (ic, qc) = code;
    mmio.write32(base + 0x0C, 0xDFF0_0220);
    mmio.write32(base + 0x10, 0x02D5_08C5);
    mmio.write32(0x09B4, 0xDB66_DB00);
    mmio.write32(base + 0xB0, 0x0A11_FB88);
    mmio.write32(base + 0xBC, 0xC008_FF81);
    mmio.write32(base + 0xC0, 0x0003_D208);
    write32_mask(mmio, base + 0xBC, 0xF000_0000, ic & 0xF);
    write32_mask(mmio, base + 0xC0, 0xF, (ic & 0xF0) >> 4);
    mmio.write32(base + 0xCC, 0x0A11_FB88);
    mmio.write32(base + 0xD8, 0xE008_FF81);
    mmio.write32(base + 0xDC, 0x0003_D208);
    write32_mask(mmio, base + 0xD8, 0xF000_0000, qc & 0xF);
    write32_mask(mmio, base + 0xDC, 0xF, (qc & 0xF0) >> 4);
    mmio.write32(base + 0xB8, 0x6000_0000);
    delay_ms(2);
    write32_mask(mmio, base + 0xBC, 0xE, 0x6);
    delay_ms(2);
    mmio.write32(base + 0xB0, 0x0A11_FB89);
    mmio.write32(base + 0xCC, 0x0A11_FB89);
    delay_ms(1);
    mmio.write32(base + 0xB8, 0x6200_0000);
    mmio.write32(base + 0xD4, 0x6200_0000);
    delay_ms(20);
    let took =
        ready(mmio, read + 0x24, 0x07F8_0000, ic) && ready(mmio, read + 0x50, 0x07F8_0000, qc);
    mmio.write32(base + 0xB8, 0x0200_0000);
    delay_ms(1);
    write32_mask(mmio, base + 0xBC, 0xE, 0x3);
    mmio.write32(0x09B4, 0xDB6D_B600);
    // The DAC's DC left, with the ADC reference biased by 0x10.
    let bias = ((adc.0 + 0x10) & 0x3FF) | (((adc.1 + 0x10) & 0x3FF) << 10);
    mmio.write32(base + 0x68, bias);
    mmio.write32(base + 0x10, 0x02D5_08C5);
    mmio.write32(base + 0x60, 0xF000_0000);
    let (i, q) = measure(|| read_sample(mmio));
    Left { ic: left(i), qc: left(q), took }
}

pub fn step4<M: Mmio>(mmio: &M, path: usize) {
    let base = write_base(path);
    mmio.write32(base + 0x68, 0);
    mmio.write32(base + 0x10, 0x02D5_08C4);
    write32_mask(mmio, base + 0xBC, 0x1, 0x0);
    write32_mask(mmio, base + 0x30, 1 << 30, 0x1);
}
