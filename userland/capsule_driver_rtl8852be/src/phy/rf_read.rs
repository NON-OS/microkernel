// NONOS Operating System (AGPL-3.0-or-later)

//! RF register reads, `rtw89_phy_read_rf_v1` (Linux v6.12 rtw89 `phy.c`):
//! a direct address reads the RF page in the baseband window; any other
//! asks the SWSI for it and waits up to 30 us for the data.

use super::bb::Bb;
use super::rf::{ADSEL, INV, RFREG_MASK, RF_BASE, R_DONE, R_SWSI_READ_ADDR_V1, R_SWSI_V1};
use crate::regs::{field, Mmio};
use crate::wait::until_us;

impl<R: Mmio> Bb<'_, R> {
    /// The value under `mask` of RF `addr` on `path`, or `INV`.
    pub fn read_rf(&self, path: u8, addr: u32, mask: u32) -> u32 {
        let Some(&base) = RF_BASE.get(path as usize) else { return INV };
        if addr & ADSEL != 0 {
            return self.get(base + ((addr & 0xff) << 2), mask & RFREG_MASK);
        }
        if !self.swsi_idle() {
            return INV;
        }
        let v = field(0x700, path as u32) | field(0xff, addr);
        self.mask(R_SWSI_READ_ADDR_V1, 0x7ff, v);
        // rtw89's 2 us pause before polling: the poll itself waits for done.
        if !until_us(30, || self.read32(R_SWSI_V1) & R_DONE != 0) {
            return INV;
        }
        self.get(R_SWSI_V1, mask & RFREG_MASK)
    }
}
