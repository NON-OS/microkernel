// NONOS Operating System (AGPL-3.0-or-later)

//! The coexistence block's indirect registers, `rtw89_mac_write_lte` and
//! `rtw89_mac_read_lte` (Linux v6.12 rtw89 `mac.c`): wait up to 50 ms for
//! the ready bit (bit 5 of `R_AX_LTE_CTRL + 3`), then issue the command.
//! rtw89 issues it even after a timeout and reports the timeout; so does
//! this.

use crate::regs::Mmio;
use crate::wait::until_us;

pub const R_AX_LTE_CTRL: u32 = 0xdaf0;
pub const R_AX_LTE_WDATA: u32 = 0xdaf4;
pub const R_AX_LTE_RDATA: u32 = 0xdaf8;
pub const R_AX_LTE_SW_CFG_1: u32 = 0x38;
pub const R_AX_LTE_SW_CFG_2: u32 = 0x3c;
const READY: u8 = 1 << 5;
const WRITE_CMD: u32 = 0xc00f_0000;
const READ_CMD: u32 = 0x800f_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LteBusy;

fn ready(r: &impl Mmio) -> Result<(), LteBusy> {
    match until_us(50_000, || r.read8(R_AX_LTE_CTRL + 3) & READY != 0) {
        true => Ok(()),
        false => Err(LteBusy),
    }
}

pub fn write(r: &impl Mmio, offset: u32, val: u32) -> Result<(), LteBusy> {
    let ok = ready(r);
    r.write32(R_AX_LTE_WDATA, val);
    r.write32(R_AX_LTE_CTRL, WRITE_CMD | offset);
    ok
}

pub fn read(r: &impl Mmio, offset: u32) -> Result<u32, LteBusy> {
    let ok = ready(r);
    r.write32(R_AX_LTE_CTRL, READ_CMD | offset);
    let v = r.read32(R_AX_LTE_RDATA);
    ok.map(|()| v)
}
