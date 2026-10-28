// NONOS Operating System (AGPL-3.0-or-later)

//! The DAC calibration's first two steps on one path (rtw88
//! `rtw8822c_dac_cal_step1` and `_step2`, rtw8822c.c:472-564): the DAC's
//! own calibration run, then the DC it leaves measured and turned into the
//! I and Q codes step 3 writes.

use super::codes::code;
use super::search::{measure, read_sample};
use super::{read_base, write_base};
use crate::phy::bits::{ready, write32_mask};
use crate::phy::rf::PATH_A;
use crate::regs::Mmio;
use crate::wait::delay_ms;

pub fn step1<M: Mmio>(mmio: &M, path: usize, adck: u32) -> bool {
    let (base, read) = (write_base(path), read_base(path));
    mmio.write32(base + 0x68, adck);
    mmio.write32(base + 0x0C, 0xDFF0_0220);
    if path == PATH_A {
        mmio.write32(base + 0x60, 0xF004_0FF0);
        mmio.write32(0x1C38, 0xFFFF_FFFF);
    }
    mmio.write32(base + 0x10, 0x02D5_08C5);
    mmio.write32(0x09B4, 0xDB66_DB00);
    for (off, v) in [(0xB0, 0x0A11_FB88), (0xBC, 0x0008_FF81), (0xC0, 0x0003_D208)] {
        mmio.write32(base + off, v);
    }
    for (off, v) in [(0xCC, 0x0A11_FB88), (0xD8, 0x0008_FF81), (0xDC, 0x0003_D208)] {
        mmio.write32(base + off, v);
    }
    mmio.write32(base + 0xB8, 0x6000_0000);
    delay_ms(2);
    mmio.write32(base + 0xBC, 0x000A_FF8D);
    delay_ms(2);
    mmio.write32(base + 0xB0, 0x0A11_FB89);
    mmio.write32(base + 0xCC, 0x0A11_FB89);
    delay_ms(1);
    mmio.write32(base + 0xB8, 0x6200_0000);
    mmio.write32(base + 0xD4, 0x6200_0000);
    delay_ms(20);
    let done =
        ready(mmio, read + 0x08, 0x7F_FF80, 0xFFFF) && ready(mmio, read + 0x34, 0x7F_FF80, 0xFFFF);
    mmio.write32(base + 0xB8, 0x0200_0000);
    delay_ms(1);
    mmio.write32(base + 0xBC, 0x0008_FF87);
    mmio.write32(0x09B4, 0xDB6D_B600);
    mmio.write32(base + 0x10, 0x02D5_08C5);
    mmio.write32(base + 0xBC, 0x0008_FF87);
    mmio.write32(base + 0x60, 0xF000_0000);
    done
}

pub fn step2<M: Mmio>(mmio: &M, path: usize) -> (u32, u32) {
    let base = write_base(path);
    write32_mask(mmio, base + 0xBC, 0xF000_0000, 0);
    write32_mask(mmio, base + 0xC0, 0xF, 0x8);
    write32_mask(mmio, base + 0xD8, 0xF000_0000, 0);
    write32_mask(mmio, base + 0xDC, 0xF, 0x8);
    mmio.write32(0x1B00, 0x0000_0008);
    mmio.write8(0x1BCC, 0x3F);
    mmio.write32(base + 0x0C, 0xDFF0_0220);
    mmio.write32(base + 0x10, 0x02D5_08C5);
    mmio.write32(0x1C3C, 0x0008_8103);
    let (i, q) = measure(|| read_sample(mmio));
    (code(i), code(q))
}
