// NONOS Operating System (AGPL-3.0-or-later)

//! The 8822C's PHY brought up for the serve loop: the board facts, the
//! front end checked against the ones rtw88 has tables for, the PCIe tail
//! rtw88 runs after the MAC init, the PHY (`phy::bring_up`), the antennas,
//! the drawn station address, the first channel and the transmit power.

use super::super::{Stage, DEFAULT_CHANNEL};
use crate::phy::channel::{set_rf, Bw};
use crate::phy::setup::rfe_is_supported;
use crate::phy::{rf, txpower};
use crate::regs::Regs;
use crate::{coex, efuse, pcie, status};

pub(super) fn phy_setup(regs: &Regs, efuse: Option<efuse::EfuseInfo>) -> Result<(), Stage> {
    // Preferably the read taken on the freshly powered MAC (`start_board`).
    let Some(info) = efuse.or_else(|| efuse::read(regs)) else {
        status::line(b"[rtl8822ce] efuse read failed, radio not configured\n");
        return Err(Stage::EfuseFailed);
    };
    if !rfe_is_supported(info.rfe) {
        status::line(
            b"[rtl8822ce] rf front-end type has no table in this driver, radio not configured\n",
        );
        return Err(Stage::EfuseFailed);
    }
    pcie::after_mac(regs, info.cut);
    if !crate::phy::bring_up(regs, &info) {
        status::line(b"[rtl8822ce] dac calibration did not read back; radio left as calibrated\n");
    }
    status::line(if rf::paths() == 2 {
        b"[rtl8822ce] phy tables loaded, two rf paths\n".as_slice()
    } else {
        b"[rtl8822ce] phy tables loaded, one rf path\n".as_slice()
    });
    if coex::take_antenna(regs) {
        status::line(b"[rtl8822ce] antenna granted to wi-fi\n");
    } else {
        status::line(b"[rtl8822ce] coex window did not answer; antenna not granted\n");
    }
    // Drawn, never read from the efuse; no entropy means no radio.
    match crate::station::draw() {
        Some(mac) => crate::mac::set_address(regs, &mac),
        None => {
            status::line(b"[rtl8822ce] no entropy for station address\n");
            return Err(Stage::NoStationAddress);
        }
    }
    set_rf(regs, DEFAULT_CHANNEL, Bw::W20);
    txpower::set_tx_power(regs, &info.power, rf::paths());
    status::line(b"[rtl8822ce] phy configured\n");
    Ok(())
}
