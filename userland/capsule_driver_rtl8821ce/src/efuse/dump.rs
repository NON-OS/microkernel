// NONOS Operating System (AGPL-3.0-or-later)

//! The physical efuse, a byte at a time, and what a stalled read left.

use core::sync::atomic::{AtomicU32, Ordering};

use super::layout::*;
use crate::regs::Mmio;

/// What the chip said when an efuse read stalled: the control register as it last
/// read back, the byte address it stalled on, and the LDO/bank register. A stage
/// name alone cannot distinguish a dead register window from a live one whose
/// efuse controller is not clocked, and this machine has no serial console, so
/// these travel to the panel in the status reply.
pub(super) static LAST_CTL: AtomicU32 = AtomicU32::new(0);
pub(super) static LAST_ADDR: AtomicU32 = AtomicU32::new(0);
pub(super) static LAST_LDO: AtomicU32 = AtomicU32::new(0);

/// The control, address and LDO registers recorded by the last stalled read.
/// All zero means no read has stalled since boot.
pub fn diag() -> (u32, u32, u32) {
    (
        LAST_CTL.load(Ordering::Relaxed),
        LAST_ADDR.load(Ordering::Relaxed),
        LAST_LDO.load(Ordering::Relaxed),
    )
}

// Read the whole physical efuse a byte at a time. Select the wifi bank and drop
// the 2.5V LDO first, then for each address clear the flag to start a read and
// spin until the hardware sets it back with the byte in the low bits.
pub(crate) fn dump_physical<M: Mmio>(mmio: &M) -> Option<[u8; EFUSE_SIZE]> {
    let bank = mmio.read32(REG_LDO_EFUSE_CTRL) & !BANK_SEL_MASK;
    mmio.write32(REG_LDO_EFUSE_CTRL, bank);
    let ldo = mmio.read8(REG_LDO_EFUSE_CTRL + 3) & !LDO25_EN;
    mmio.write8(REG_LDO_EFUSE_CTRL + 3, ldo);

    let mut map = [0u8; EFUSE_SIZE];
    let mut ctl = mmio.read32(REG_EFUSE_CTRL);
    for (addr, slot) in map.iter_mut().enumerate() {
        ctl &= !(EF_DATA_MASK | (EF_ADDR_MASK << EF_ADDR_SHIFT));
        ctl |= (addr as u32 & EF_ADDR_MASK) << EF_ADDR_SHIFT;
        mmio.write32(REG_EFUSE_CTRL, ctl & !EF_FLAG);

        let mut spins = 1_000_000u32;
        loop {
            ctl = mmio.read32(REG_EFUSE_CTRL);
            if ctl & EF_FLAG != 0 {
                break;
            }
            spins -= 1;
            if spins == 0 {
                LAST_CTL.store(ctl, Ordering::Relaxed);
                // Stored one past the address so zero keeps meaning "no read has
                // stalled". A window that reads back all-zero would otherwise be
                // indistinguishable from a loop that was never entered, and those
                // are opposite faults.
                LAST_ADDR.store(addr as u32 + 1, Ordering::Relaxed);
                LAST_LDO.store(mmio.read32(REG_LDO_EFUSE_CTRL), Ordering::Relaxed);
                return None;
            }
        }
        *slot = (ctl & EF_DATA_MASK) as u8;
    }
    Some(map)
}
