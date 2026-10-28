// NONOS Operating System (AGPL-3.0-or-later)

//! The PCIe PHY's MDIO registers through the MAC: `rtw89_pci_check_mdio`,
//! `rtw89_read16_mdio`, `rtw89_write16_mdio` and the mask form (Linux v6.12
//! rtw89 `pci.c`). The page select picks GEN1 or GEN2 and the low or high
//! 32 registers; the read or write flag clears when the PHY is done (2 ms).

use crate::regs::{field, Mmio, Rw};
use crate::wait::until_us;

pub const R_AX_MDIO_CFG: u32 = 0x10a0;
pub const R_AX_MDIO_WDATA: u32 = 0x10a4;
pub const R_AX_MDIO_RDATA: u32 = 0x10a6;
const PHY_ADDR_MASK: u16 = 0x3000;
pub const RFLAG: u16 = 0x200;
pub const WFLAG: u16 = 0x100;
pub const GEN1: u8 = 0;
pub const GEN2: u8 = 1;

/// The PHY did not finish an MDIO access in time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MdioTimeout;

/// The page select for `addr` at `speed`: MDIO_PG{0,1}_G{1,2}.
pub const fn page(addr: u8, speed: u8) -> u16 {
    let high = if addr < 0x20 { 0 } else { 1 };
    (speed as u16) * 2 + high
}

fn start(r: &impl Mmio, addr: u8, speed: u8, flag: u16) -> Result<(), MdioTimeout> {
    r.write8(R_AX_MDIO_CFG, addr & 0x1f);
    let v = (r.read16(R_AX_MDIO_CFG) & !PHY_ADDR_MASK)
        | field(PHY_ADDR_MASK as u32, page(addr, speed) as u32) as u16;
    r.write16(R_AX_MDIO_CFG, v);
    r.set16(R_AX_MDIO_CFG, flag);
    let done = until_us(2000, || r.read16(R_AX_MDIO_CFG) & flag == 0);
    done.then_some(()).ok_or(MdioTimeout)
}

pub fn read16(r: &impl Mmio, addr: u8, speed: u8) -> Result<u16, MdioTimeout> {
    start(r, addr, speed, RFLAG)?;
    Ok(r.read16(R_AX_MDIO_RDATA))
}

pub fn write16(r: &impl Mmio, addr: u8, data: u16, speed: u8) -> Result<(), MdioTimeout> {
    r.write16(R_AX_MDIO_WDATA, data);
    start(r, addr, speed, WFLAG)
}

/// `rtw89_write16_mdio_mask`: read, place `data` under `mask`, write back.
pub fn write16_mask(
    r: &impl Mmio,
    addr: u8,
    mask: u16,
    data: u16,
    speed: u8,
) -> Result<(), MdioTimeout> {
    let v = read16(r, addr, speed)?;
    let v = (v & !mask) | field(mask as u32, data as u32) as u16;
    write16(r, addr, v, speed)
}
