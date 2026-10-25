// NONOS Operating System (AGPL-3.0-or-later)

//! Load the baseband, AGC, RF and PHY-MAC tables into the chip. Each table is
//! walked by the condition parser; a matching entry is a register write, except
//! for a small set of reserved addresses the vendor uses as inline settle
//! delays. The MAC table writes bytes, the BB and AGC tables write 32-bit
//! registers, and the RF table writes the path-A register through the serial
//! (SIPI) interface. The write widths, the delay addresses and the SIPI word
//! layout follow rtw88 `rtw_phy_cfg_mac/agc/bb/rf` and
//! `rtw_phy_write_rf_reg_sipi`; the SIPI encoding is checked in `rtl8821ce_proofs`.

mod delay;
mod load;

pub use load::*;
