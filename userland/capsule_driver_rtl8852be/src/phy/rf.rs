// NONOS Operating System (AGPL-3.0-or-later)

//! RF register access, `rtw89_phy_write_rf_v1` / `rtw89_phy_read_rf_v1`
//! (Linux v6.12 rtw89 `phy.c`). An address with bit 16 set is a direct
//! one: the RF page at `rf_base_addr` (0xe000 for path A, 0xf000 for B, in
//! the baseband window) plus four bytes per register. Any other goes
//! through the software serial interface (SWSI), which must be idle first
//! (30 us); reads are in `rf_read`.

use super::bb::Bb;
use crate::regs::{field, Mmio};
use crate::wait::until_us;

pub const RFREG_MASK: u32 = 0xfffff;
pub const ADSEL: u32 = 1 << 16;
pub const RF_BASE: [u32; 2] = [0xe000, 0xf000];
pub const R_SWSI_V1: u32 = 0x174c;
const W_BUSY: u32 = 0x100_0000;
const R_BUSY: u32 = 0x200_0000;
pub const R_DONE: u32 = 0x400_0000;
pub const R_SWSI_DATA_V1: u32 = 0x370;
pub const R_SWSI_BIT_MASK_V1: u32 = 0x374;
pub const R_SWSI_READ_ADDR_V1: u32 = 0x378;
/// The bad-read answer, `INV_RF_DATA`.
pub const INV: u32 = u32::MAX;

impl<R: Mmio> Bb<'_, R> {
    pub(super) fn swsi_idle(&self) -> bool {
        until_us(30, || self.read32(R_SWSI_V1) & (W_BUSY | R_BUSY) == 0)
    }

    /// Write `data` under `mask` to RF `addr` on `path`; false when SWSI
    /// stayed busy or the path does not exist.
    pub fn write_rf(&self, path: u8, addr: u32, mask: u32, data: u32) -> bool {
        let Some(&base) = RF_BASE.get(path as usize) else { return false };
        if addr & ADSEL != 0 {
            let reg = base + ((addr & 0xff) << 2);
            self.mask(reg, mask & RFREG_MASK, data);
            // rtw89 waits 1 us "to ensure writing properly"; the clock here is
            // no finer than 1 ms. Reading the register back completes the
            // write at the card (a read cannot pass a posted write) and takes
            // at least a bus round trip, which is the microsecond asked for.
            let _ = self.read32(reg);
            return true;
        }
        if !self.swsi_idle() {
            return false;
        }
        let (mask, mut data) = (mask & RFREG_MASK, data & RFREG_MASK);
        let masked = mask != RFREG_MASK;
        if masked {
            self.mask(R_SWSI_BIT_MASK_V1, RFREG_MASK, mask);
            data = (data << mask.trailing_zeros()) & RFREG_MASK;
        }
        let v = field(0x8000_0000, masked as u32)
            | field(0x7000_0000, path as u32)
            | field(0x0ff0_0000, addr)
            | field(0x000f_ffff, data);
        self.mask(R_SWSI_DATA_V1, u32::MAX, v);
        true
    }
}
