// NONOS Operating System (AGPL-3.0-or-later)

//! Port 0 for the station: `rtw89_mac_port_update` (Linux v6.12 rtw89
//! `mac.c`) as its `rtw89_mac_port_cfg_*` steps program a client of no
//! AP (before association) or of one (after), never an AP: beacon reports
//! off, the net type, beacon protection and BSSID fit and TSF sync only
//! once joined, beacon TX off, the beacon interval, HIQ off, the beacon
//! timing defaults, no BSS colour or MBSSID, the port on. The station
//! passes no DTIM period, so the DTIM count stays 0, rtw89's value before
//! association.

use crate::h2c::media::Net;
use crate::regs::{Mmio, Rw};
use crate::wait::delay_ms;

const PORT_CFG: u32 = 0xc400;
const FUNC_EN: u32 = 0x4;
const BRK_SETUP: u32 = 0x1_0000;
const TBTT_PROHIB_EN: u32 = 0x2000;
const RX_BSSID_FIT_EN: u32 = 0x10;
const TSF_UDT_EN: u32 = 0x8;
const TBTT_PROHIB: u32 = 0xc404;

/// `rtw89_mac_port_cfg_func_sw`: an enabled port is stopped a beacon
/// interval (plus 1 ms) after its last use and its TSF reset.
fn func_sw(r: &impl Mmio, beacon_int: u16) {
    if r.read32(PORT_CFG) & FUNC_EN == 0 {
        return;
    }
    delay_ms(beacon_int as u64 + 1);
    r.clr32(PORT_CFG, FUNC_EN | BRK_SETUP);
    r.set32(PORT_CFG, 0x20); // TSFTR_RST
    r.write32(0xc434, 0); // BCN_CNT_TMR
}

fn bits(r: &impl Mmio, bits: u32, on: bool) {
    if on {
        r.set32(PORT_CFG, bits);
    } else {
        r.clr32(PORT_CFG, bits);
    }
}

/// `beacon_int` is the AP's, or 0 before one is known (then 100 TU).
pub fn port_update(r: &impl Mmio, net: Net, beacon_int: u16) {
    let joined = net == Net::Infra;
    func_sw(r, beacon_int);
    r.clr32(PORT_CFG, 0x2); // tx_rpt off
    r.clr32(PORT_CFG, 0x1); // rx_rpt off
    r.mask32(PORT_CFG, 0xc00, net as u32);
    bits(r, TBTT_PROHIB_EN | BRK_SETUP, joined);
    bits(r, RX_BSSID_FIT_EN, joined);
    bits(r, TSF_UDT_EN, joined);
    r.clr32(PORT_CFG, 0x1000); // tx_sw: no beacons sent
    let bi = if beacon_int == 0 { 100 } else { beacon_int };
    r.mask32(0xc414, 0xffff, bi as u32); // BCN_SPACE
    r.write8(0xc590, 0); // HIQ window
    r.set8(0xca08, 0x2 | 0x1); // UPD_HGQMD | UPD_TIMIE
    r.mask16(0xc426, 0xff00, 0); // DTIM_NUM
    let drop = r.read32(0xc63c) & !(0x1_0000 | 0x1); // port 0's drop bits
    r.write32(0xc63c, drop);
    r.mask32(TBTT_PROHIB, 0xff, 0x2); // BCN_SETUP_DEF
    r.mask32(TBTT_PROHIB, 0xfff_0000, 0xc8); // BCN_HOLD_DEF
    r.mask32(0xc408, 0xfff_0000, 0); // BCN_MASK_DEF
    r.mask16(0xc40e, 0xfff, 0x5); // TBTT_ERLY_DEF
    r.mask32(0xc6a0, 0x3f, 0); // BSS colour 0
    r.clr32(0xc568, 0xff_fffe); // MBSSID off
    r.set32(PORT_CFG, FUNC_EN);
    delay_ms(1); // BCN_ERLY_SET_DLY, 20 us
    r.mask32(0xc40c, 0xfff, 0xa0); // BCN_ERLY_DEF
}
