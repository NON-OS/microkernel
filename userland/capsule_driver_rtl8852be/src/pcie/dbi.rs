// NONOS Operating System (AGPL-3.0-or-later)

//! The chip's own window onto its PCIe configuration space:
//! `rtw89_dbi_read8` / `rtw89_dbi_write8` (Linux v6.12 rtw89 `pci.c`).
//! rtw89 tries the host's config access first and falls back to this for
//! the 8852B; the driver goes straight to it, which reads and writes the
//! same registers without asking the broker for config-space authority.

use crate::regs::{field, Mmio};
use crate::wait::until_us;

pub const R_AX_DBI_FLAG: u32 = 0x1090;
pub const R_AX_DBI_WDATA: u32 = 0x1094;
pub const R_AX_DBI_RDATA: u32 = 0x1098;
const ADDR_MSK: u16 = 0xffc;
const WREN_MSK: u16 = 0xf000;
/// The write and read flags, in the flag register's third byte.
pub const WFLAG: u8 = 0x1;
pub const RFLAG: u8 = 0x2;
/// rtw89's poll: 10 us apart, 10 * RTW89_PCI_WR_RETRY_CNT us in all.
const POLL_US: u64 = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbiTimeout;

fn wait(r: &impl Mmio) -> Result<(), DbiTimeout> {
    let ok = until_us(POLL_US, || r.read8(R_AX_DBI_FLAG + 2) == 0);
    ok.then_some(()).ok_or(DbiTimeout)
}

pub fn read8(r: &impl Mmio, addr: u16) -> Result<u8, DbiTimeout> {
    r.write16(R_AX_DBI_FLAG, addr & ADDR_MSK);
    r.write8(R_AX_DBI_FLAG + 2, RFLAG);
    wait(r)?;
    Ok(r.read8(R_AX_DBI_RDATA + (addr & 3) as u32))
}

pub fn write8(r: &impl Mmio, addr: u16, data: u8) -> Result<(), DbiTimeout> {
    let lsb = addr & 3;
    let cmd = (addr & ADDR_MSK) | field(WREN_MSK as u32, 1 << lsb) as u16;
    r.write8(R_AX_DBI_WDATA + lsb as u32, data);
    r.write16(R_AX_DBI_FLAG, cmd);
    r.write8(R_AX_DBI_FLAG + 2, WFLAG);
    wait(r)
}
