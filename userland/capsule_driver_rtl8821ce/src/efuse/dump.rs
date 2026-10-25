// NONOS Operating System (AGPL-3.0-or-later)

//! The physical efuse, a byte at a time, and what a stalled read left.

use nonos_wifi_core::vault::wipe;

use super::diag::record_stall;
use super::layout::*;
use crate::regs::Mmio;
use crate::wait::until_ms;

/// How long one byte's read may take (rtw88 reads a million times, a
/// microsecond apart).
const BYTE_MS: u64 = 1000;

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

        let read = until_ms(BYTE_MS, || {
            ctl = mmio.read32(REG_EFUSE_CTRL);
            ctl & EF_FLAG != 0
        });
        if !read {
            // One past the address, so a window that reads back all-zero is
            // told apart from a loop that was never entered.
            record_stall(ctl, addr, mmio.read32(REG_LDO_EFUSE_CTRL));
            wipe(&mut map);
            return None;
        }
        *slot = (ctl & EF_DATA_MASK) as u8;
    }
    Some(map)
}
