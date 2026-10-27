// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_dump_physical_efuse_map_dav` (Linux v6.12 rtw89 `efuse.c`): the
//! DAV cells, read one by one through the crystal block's serial port.

use super::dump::EfuseTimeout;
use crate::pwr::xtal;
use crate::regs::Mmio;
use crate::wait::until_us;

const XTAL_SI_LOW_ADDR: u8 = 0x62;
const XTAL_SI_CTRL: u8 = 0x63;
const XTAL_SI_READ_VAL: u8 = 0x7a;
const SI_RDY: u8 = 0x20;

/// The DAV cells, one by one through the crystal block (10 ms each).
pub fn dav(r: &impl Mmio, map: &mut [u8]) -> Result<(), EfuseTimeout> {
    let t = |_| EfuseTimeout;
    for (addr, b) in map.iter_mut().enumerate() {
        xtal::write(r, XTAL_SI_CTRL, 0x40, 0xff).map_err(t)?;
        xtal::write(r, XTAL_SI_LOW_ADDR, addr as u8, 0xff).map_err(t)?;
        xtal::write(r, XTAL_SI_CTRL, (addr >> 8) as u8, 0x07).map_err(t)?;
        xtal::write(r, XTAL_SI_CTRL, 0, 0xc0).map_err(t)?;
        let ready = || xtal::read(r, XTAL_SI_CTRL).is_ok_and(|v| v & SI_RDY != 0);
        if !until_us(10_000, ready) {
            return Err(EfuseTimeout);
        }
        *b = xtal::read(r, XTAL_SI_READ_VAL).map_err(t)?;
    }
    Ok(())
}
