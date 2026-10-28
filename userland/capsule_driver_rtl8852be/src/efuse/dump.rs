// NONOS Operating System (AGPL-3.0-or-later)

//! Reading the efuse cells: `rtw89_dump_physical_efuse_map_ddv` with its
//! power-cut enable and disable (Linux v6.12 rtw89 `efuse.c`; the DAV cells
//! are in `dav`). Each DDV cell is asked for by address and
//! comes back in the control word's low byte once its ready bit sets (1 s).

use crate::regs::{field, Mmio, Rw};
use crate::wait::{delay_ms, until_us};

pub const R_AX_EFUSE_CTRL: u32 = 0x30;
pub const EF_ADDR: u32 = 0x7ff_0000;
pub const EF_RDY: u32 = 0x2000_0000;
pub const R_AX_EFUSE_CTRL_1_V1: u32 = 0x38;
pub const EF_BURST: u32 = 0x8_0000;
const R_AX_PMC_DBG_CTRL2: u32 = 0xcc;
const WRMSK: u8 = 0x4;
const R_AX_SYS_ISO_CTRL: u32 = 0x0;
const EV2EF_B14: u16 = 0x4000;
const EV2EF_B15: u16 = 0x8000;
const EB2CORE: u16 = 0x100;
/// `CHIP_CAV`: the cut that reads in burst mode.
const CAV: u8 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfuseTimeout;

/// Fill `map` from the cells at `start`, `start + 1`, ...
pub fn ddv(r: &impl Mmio, cv: u8, start: u32, map: &mut [u8]) -> Result<(), EfuseTimeout> {
    r.set8(R_AX_PMC_DBG_CTRL2, WRMSK);
    r.set16(R_AX_SYS_ISO_CTRL, EV2EF_B14);
    delay_ms(1);
    r.set16(R_AX_SYS_ISO_CTRL, EV2EF_B15);
    r.clr16(R_AX_SYS_ISO_CTRL, EB2CORE);
    if cv == CAV {
        r.set32(R_AX_EFUSE_CTRL_1_V1, EF_BURST);
    }
    for (i, b) in map.iter_mut().enumerate() {
        let ctl = field(EF_ADDR, start + i as u32);
        r.write32(R_AX_EFUSE_CTRL, ctl & !EF_RDY);
        let mut v = 0;
        if !until_us(1_000_000, || {
            v = r.read32(R_AX_EFUSE_CTRL);
            v & EF_RDY != 0
        }) {
            return Err(EfuseTimeout);
        }
        *b = v as u8;
    }
    if cv == CAV {
        r.clr32(R_AX_EFUSE_CTRL_1_V1, EF_BURST);
    }
    r.set16(R_AX_SYS_ISO_CTRL, EB2CORE);
    r.clr16(R_AX_SYS_ISO_CTRL, EV2EF_B15);
    delay_ms(1);
    r.clr16(R_AX_SYS_ISO_CTRL, EV2EF_B14);
    r.clr8(R_AX_PMC_DBG_CTRL2, WRMSK);
    Ok(())
}
