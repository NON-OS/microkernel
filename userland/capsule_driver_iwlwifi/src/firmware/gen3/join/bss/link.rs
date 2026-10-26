// NONOS Operating System (AGPL-3.0-or-later)

//! The link's parameters for the access point, as each step names them.

use super::super::super::station::link::LinkParams;
use super::Bss;

/// `WLAN_CAPABILITY_SHORT_PREAMBLE` and `_SHORT_SLOT_TIME`.
const CAP_SHORT_PREAMBLE: u16 = 1 << 5;
const CAP_SHORT_SLOT: u16 = 1 << 10;

impl Bss {
    pub(super) fn link(
        &self,
        phy: Option<u32>,
        active: bool,
        modify_mask: u32,
        capability: u16,
    ) -> LinkParams {
        let t = &self.target;
        LinkParams {
            addr: self.addr,
            phy,
            active,
            modify_mask,
            cck_ack: t.rates.cck_ack,
            ofdm_ack: t.rates.ofdm_ack,
            short_preamble: capability & CAP_SHORT_PREAMBLE != 0,
            short_slot: t.channel > 14 || capability & CAP_SHORT_SLOT != 0,
            beacon_int: t.beacon_int,
            dtim_period: t.dtim_period,
        }
    }
}
