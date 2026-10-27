// NONOS Operating System (AGPL-3.0-or-later)

//! The 8822C's RF registers, both paths. A read is direct, from the path's
//! window at `rf_base_addr` (0x3C00, 0x4C00) plus the register number times
//! four (rtw88 `rtw_phy_read_rf`). A write is `rtw_phy_write_rf_reg_mix`:
//! direct into the same window for every register but 0, and through the
//! path's serial interface (`rf_sipi_addr` 0x1808, 0x4108) for register 0.
//! A path the chip does not have (`set_paths`) is neither read nor written,
//! as rtw88 refuses a path at or past `rf_phy_num`.

use core::sync::atomic::{AtomicU8, Ordering};

use super::bits::write32_mask;
use crate::regs::Mmio;

/// `RFREG_MASK`: the 20 bits of an RF register.
pub const RFREG_MASK: u32 = 0x000F_FFFF;
/// `INV_RF_DATA`: what reading a path the chip lacks returns.
pub const INV_RF_DATA: u32 = 0xFFFF_FFFF;
pub const PATH_A: usize = 0;
pub const PATH_B: usize = 1;

const BASE: [usize; 2] = [0x3C00, 0x4C00];
const SIPI: [usize; 2] = [0x1808, 0x4108];

/// The chip's RF paths, from `REG_SYS_CFG1` at bring-up; 2 until then.
static PATHS: AtomicU8 = AtomicU8::new(2);

pub fn set_paths(n: u8) {
    PATHS.store(n.clamp(1, 2), Ordering::Relaxed);
}

pub fn paths() -> usize {
    PATHS.load(Ordering::Relaxed) as usize
}

/// The field under `mask` of RF register `addr` on `path`.
pub fn read<M: Mmio>(mmio: &M, path: usize, addr: u8, mask: u32) -> u32 {
    if path >= paths() {
        return INV_RF_DATA;
    }
    let mask = mask & RFREG_MASK;
    (mmio.read32(BASE[path] + ((addr as usize) << 2)) & mask) >> (mask.trailing_zeros() % 32)
}

/// Write the field under `mask` of RF register `addr` on `path`; false for a
/// path the chip lacks.
pub fn write<M: Mmio>(mmio: &M, path: usize, addr: u8, mask: u32, data: u32) -> bool {
    if path >= paths() {
        return false;
    }
    let mask = mask & RFREG_MASK;
    if addr != 0 {
        write32_mask(mmio, BASE[path] + ((addr as usize) << 2), mask, data);
        settle(1);
        return true;
    }
    let mut data = data;
    if mask != RFREG_MASK {
        let old = read(mmio, path, 0, RFREG_MASK);
        data = (old & !mask) | (data << (mask.trailing_zeros() % 32));
    }
    mmio.write32(SIPI[path], (data & RFREG_MASK) & 0x0FFF_FFFF);
    settle(13);
    true
}

/// rtw88's microsecond settle after an RF write: below the clock's tick, so
/// a short spin, a delay rather than a wait for anything.
fn settle(us: u32) {
    for _ in 0..us * 256 {
        core::hint::spin_loop();
    }
}
