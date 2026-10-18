// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY brought up: power, tables, antenna, station address, channel.

use super::super::{Stage, DEFAULT_CHANNEL};
use crate::efuse;
use crate::phy::channel::Bw;
use crate::phy::{load_all, power_on, rxpath, PhyCond, INTF_PCIE};
use crate::regs::Regs;
use crate::status;

// Bring the radio up: read the board facts the PHY needs from the efuse, load the
// MAC, baseband, AGC and RF register tables for this cut and RF front-end, hold the
// receive path in reset across the table load, then select the 2.4GHz receive path,
// route the antenna into the receive amplifier, set transmit power, and tune the
// default channel. Without a good efuse read the RF front-end is unknown, so the
// radio is left unconfigured rather than programmed with the wrong tables.
pub(super) fn phy_setup(regs: &Regs, efuse: Option<efuse::EfuseInfo>) -> Result<(), Stage> {
    // Taken during bring-up, on a freshly powered MAC, which is where rtw88 reads
    // it. Read here instead, after the firmware download and the MAC tables, the
    // efuse control and LDO registers answered zero on real silicon.
    // Preferably the read taken during bring-up, on a freshly powered MAC, which
    // is where rtw88 takes it. If that came back unreadable, try again here, which
    // is where this driver read it when the radio last came up on real hardware.
    // Never fall through with a map that failed validation: an all-ones efuse
    // yields front-end 0x1F, and programming the RF tables from it wedges the chip
    // hard enough that its register window stops answering at all.
    let info = match efuse.or_else(|| efuse::read(regs)) {
        Some(info) => info,
        None => {
            status::line(b"[rtl8821ce] efuse read failed, radio not configured\n");
            return Err(Stage::EfuseFailed);
        }
    };
    // rtw88 refuses a front-end it has no table for rather than programming the
    // nearest one. The radio tables are indexed off this, so a wrong value is not
    // a degraded radio, it is a chip driven with settings for hardware that is not
    // on this board.
    if !rxpath::rfe_is_supported(info.rfe) {
        status::line(b"[rtl8821ce] unsupported rf front-end, radio not configured\n");
        return Err(Stage::EfuseFailed);
    }
    let cond = PhyCond { cut: info.cut, pkg: info.pkg, intf: INTF_PCIE, rfe: info.rfe };
    power_on(regs);
    rxpath::pre_tables(regs);
    load_all(regs, &cond);
    rxpath::post_tables(regs);
    // The antenna is shared with Bluetooth; rtw88 hands it to Wi-Fi here, after
    // the tables and before the first channel. Without it the receiver can sit
    // on Bluetooth's side of the switch and hear nothing.
    if crate::coex::take_antenna(regs, info.rfe) {
        status::line(b"[rtl8821ce] antenna granted to wi-fi\n");
    } else {
        status::line(b"[rtl8821ce] coex window did not answer; antenna not granted\n");
    }
    // The chip does not autoload a MAC, and an all-zero one associates but never
    // completes the four-way, so this must happen after the table load. Drawn
    // rather than read from the efuse; no entropy means no radio.
    match crate::station::draw() {
        Some(mac) => crate::mac::set_address(regs, &mac),
        None => {
            status::line(b"[rtl8821ce] no entropy for station address\n");
            return Err(Stage::NoStationAddress);
        }
    }
    let rfe_btg = rxpath::rfe_is_btg(info.rfe);
    rxpath::set_channel(regs, DEFAULT_CHANNEL, Bw::W20, rfe_btg);
    status::line(b"[rtl8821ce] phy configured\n");
    Ok(())
}
