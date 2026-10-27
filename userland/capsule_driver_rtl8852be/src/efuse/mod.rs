// NONOS Operating System (AGPL-3.0-or-later)

//! The board's efuse: `rtw89_parse_efuse_map_ax` and
//! `rtw89_parse_phycap_map_ax` (Linux v6.12 rtw89 `efuse.c`), run while the
//! firmware is up after the first download, as `rtw89_chip_efuse_info_setup`
//! runs them. The DAV map is read and decoded as rtw89 does; the 8852B's
//! fields all sit in the main map.

pub mod dav;
pub mod dump;
pub mod logical;
pub mod map;
pub mod phycap;

use crate::regs::Mmio;
use map::*;

pub const R_AX_SYS_WL_EFUSE_CTRL: u32 = 0xa;
pub const B_AX_AUTOLOAD_SUS: u16 = 0x20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfuseFail {
    Timeout,
    Corrupt,
    Short,
}

/// What the efuse holds, and whether the chip autoloaded it (`efuse.valid`).
pub struct Board {
    pub valid: bool,
    pub efuse: Efuse,
    pub phycap: phycap::Phycap,
}

pub fn read(r: &impl Mmio, cv: u8) -> Result<Board, EfuseFail> {
    let valid = r.read16(R_AX_SYS_WL_EFUSE_CTRL) & B_AX_AUTOLOAD_SUS != 0;
    let mut phy = [0u8; PHYSICAL_SIZE];
    dump::ddv(r, cv, 0, &mut phy).map_err(|_| EfuseFail::Timeout)?;
    // The DAV cells, read into a buffer of the main map's size whose tail
    // stays 0xff: rtw89 walks them with the main map's physical bounds. A
    // record landing outside the 16-byte DAV map is refused, where rtw89
    // would write past its buffer.
    let mut dav_map = [0xffu8; PHYSICAL_SIZE];
    dav::dav(r, &mut dav_map[..DAV_PHYSICAL_SIZE]).map_err(|_| EfuseFail::Timeout)?;
    let mut log = [0xffu8; LOGICAL_SIZE + DAV_LOGICAL_SIZE];
    let bad = |_| EfuseFail::Corrupt;
    logical::logical(&phy, SEC_CTRL_SIZE, &mut log[..LOGICAL_SIZE]).map_err(bad)?;
    logical::logical(&dav_map, SEC_CTRL_SIZE, &mut log[LOGICAL_SIZE..]).map_err(bad)?;
    let efuse = map::parse(&log).ok_or(EfuseFail::Short)?;
    let mut cap = [0u8; phycap::PHYCAP_SIZE];
    dump::ddv(r, cv, phycap::PHYCAP_ADDR, &mut cap).map_err(|_| EfuseFail::Timeout)?;
    Ok(Board { valid, efuse, phycap: phycap::parse(&cap) })
}
