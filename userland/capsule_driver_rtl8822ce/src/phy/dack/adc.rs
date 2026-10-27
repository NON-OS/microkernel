// NONOS Operating System (AGPL-3.0-or-later)

//! The ADC DC calibration of one path (rtw88 `rtw8822c_dac_cal_adc`,
//! rtw8822c.c:394-470): at most ten rounds measuring the receive DC with the
//! RF in its calibration mode, writing the compensation, measuring again,
//! until both I and Q are within 5. Returns the compensation the DAC steps
//! take as their reference, and the word written (`dack_adck`).

use super::codes::{compensation, magnitude};
use super::search::{measure, read_sample};
use super::{write_base, ADCK_ROUNDS};
use crate::phy::bits::write32_mask;
use crate::phy::rf::{self, PATH_A, PATH_B, RFREG_MASK};
use crate::regs::Mmio;

/// The ADC compensation found: (I, Q) and the word written at base + 0x68.
pub struct Adck {
    pub ic: u32,
    pub qc: u32,
    pub word: u32,
}

pub fn adc<M: Mmio>(mmio: &M, path: usize) -> Adck {
    let base = write_base(path);
    let path_sel = if path == PATH_A { 0xA_0000 } else { 0x8_0000 };
    write32_mask(mmio, base + 0x30, 1 << 30, 0);
    if path == PATH_B {
        mmio.write32(base + 0x30, 0x30DB_8041);
    }
    mmio.write32(base + 0x60, 0xF004_0FF0);
    mmio.write32(base + 0x0C, 0xDFF0_0220);
    mmio.write32(base + 0x10, 0x02DD_08C4);
    mmio.write32(base + 0x0C, 0x1000_0260);
    rf::write(mmio, PATH_A, 0x0, RFREG_MASK, 0x10000);
    rf::write(mmio, PATH_B, 0x0, RFREG_MASK, 0x10000);
    let mut out = Adck { ic: 0, qc: 0, word: 0 };
    for _ in 0..ADCK_ROUNDS {
        mmio.write32(0x1C3C, path_sel + 0x8003);
        mmio.write32(0x1C24, 0x0001_0002);
        let (ic, qc) = measure(|| read_sample(mmio));
        let (ic, qc) = (compensation(ic), compensation(qc));
        if ic != 0 {
            out.ic = ic;
        }
        if qc != 0 {
            out.qc = qc;
        }
        out.word = (ic & 0x3FF) | ((qc & 0x3FF) << 10);
        mmio.write32(base + 0x68, out.word);
        mmio.write32(0x1C3C, path_sel + 0x8103);
        let (ic, qc) = measure(|| read_sample(mmio));
        if magnitude(ic) < 5 && magnitude(qc) < 5 {
            break;
        }
    }
    mmio.write32(0x1C3C, 0x0000_0003);
    mmio.write32(base + 0x0C, 0x1000_0260);
    mmio.write32(base + 0x10, 0x02D5_08C4);
    rf::write(mmio, path, 0x8F, 1 << 13, 0x1);
    out
}
