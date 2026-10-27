// NONOS Operating System (AGPL-3.0-or-later)

//! One frame out, as `rtw89_core_tx_write` routes it (Linux v6.12 rtw89
//! `core.c`): management frames (and null functions) on the management
//! queue at a fixed rate, data on the best-effort queue with the firmware
//! choosing the rate. A management frame to the AP once one is chosen goes
//! at its lowest basic rate (mac80211's `basic_rates`, read here from the
//! AP's beacons); any other at the band's lowest rate. A data frame's
//! lowest retry rate is the lowest rate the AP supports.

use super::chip::Chip;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::classify::classify;
use crate::tx::rate::{bitmaps, lowest, pick};
use crate::tx::wd::build;
use crate::tx::wd_ring::Full;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotSent {
    /// Not a frame this driver sends (short, control, or too long).
    Refused,
    NoChannel,
    Ring(Full),
}

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    pub fn send(&mut self, mpdu: &[u8]) -> Result<(), NotSent> {
        let mut d = classify(mpdu).ok_or(NotSent::Refused)?;
        let five = self.chan.ok_or(NotSent::NoChannel)?.five;
        let to_ap = self.vif.bssid != [0; 6] && mpdu.get(4..10) == Some(&self.vif.bssid[..]);
        let (supp, basic) = bitmaps(self.vif.rates(), five);
        d.rate = match (d.mgmt, to_ap) {
            (true, true) => pick(basic, five),
            (false, true) => pick(supp, five),
            _ => lowest(five),
        };
        let wd = build(&d);
        let ring = if d.mgmt { &mut self.rings.ch8 } else { &mut self.rings.ach0 };
        ring.send_frame(&self.r, &wd, mpdu).map_err(NotSent::Ring)
    }
}
