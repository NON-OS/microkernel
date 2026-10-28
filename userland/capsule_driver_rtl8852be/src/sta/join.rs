// NONOS Operating System (AGPL-3.0-or-later)

//! Joining and leaving an AP, in the order mac80211 drives rtw89 (Linux
//! v6.12 rtw89 `mac80211.c`, `core.c`): the BSSID into the CAM
//! (`rtw89_ops_link_info_changed`), then the AP added as a station, which
//! calibrates for the channel (`rtw89_core_sta_add`: RX DCK and IQK, the
//! part of `rtw8852b_rfk_channel` ported); on association the CMAC table,
//! join info, CAM with the AID, rate adaptation and the port
//! (`rtw89_core_sta_assoc`, `rtw89_ops_vif_cfg_changed`); on leaving the
//! reverse (`rtw89_core_sta_disconnect`), the port back to no link, which
//! rtw89 leaves to the interface going down, and the CAM entry without
//! its keys.

use super::chip::Chip;
use crate::bring::Stop;
use crate::dma::DmaMem;
use crate::h2c::cam_keys::Keys;
use crate::h2c::cctl::{assoc_tbl, send};
use crate::h2c::media::{join_info, Net};
use crate::h2c::ra::ra_assoc;
use crate::mac::port::port_update;
use crate::regs::Mmio;
use crate::rfk;

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    pub fn join(&mut self, bssid: [u8; 6]) -> Result<(), Stop> {
        self.scan_end();
        self.vif.bssid = bssid;
        self.vif.n_rates = 0;
        self.cam_update(self.vif.sma).map_err(Stop::Send)?;
        let c = self.chan.ok_or(Stop::NoChannel)?;
        rfk::channel(&self.r, &mut self.fw, &mut self.rfk, c).map_err(Stop::Sch)
    }

    /// `rates` and `beacon_int` are the association's.
    pub fn associated(&mut self, aid: u16, rates: &[u8], beacon_int: u16) -> Result<(), Stop> {
        let five = self.chan.is_some_and(|c| c.five);
        let (r, q, fw) = (&self.r, &mut self.rings.ch12, &mut self.fw);
        send(r, q, fw, &assoc_tbl(five)).map_err(Stop::Send)?;
        self.vif.net = Net::Infra;
        self.vif.aid = aid;
        self.vif.beacon_int = beacon_int;
        join_info(r, q, fw, false, Net::Infra).map_err(Stop::Send)?;
        self.cam_update(self.vif.sma).map_err(Stop::Send)?;
        let (r, q, fw) = (&self.r, &mut self.rings.ch12, &mut self.fw);
        ra_assoc(r, q, fw, rates, five).map_err(Stop::Send)?;
        port_update(r, Net::Infra, beacon_int);
        Ok(())
    }

    pub fn leave(&mut self) -> Result<(), Stop> {
        let five = self.chan.is_some_and(|c| c.five);
        if self.vif.net == Net::Infra {
            let (r, q, fw) = (&self.r, &mut self.rings.ch12, &mut self.fw);
            send(r, q, fw, &assoc_tbl(five)).map_err(Stop::Send)?;
            join_info(r, q, fw, true, Net::NoLink).map_err(Stop::Send)?;
            port_update(r, Net::NoLink, self.vif.beacon_int);
        }
        self.vif.net = Net::NoLink;
        self.vif.aid = 0;
        self.vif.bssid = [0; 6];
        // The keys detached, as `rtw89_cam_sec_key_del` detaches each.
        self.vif.keys = Keys::default();
        self.cam_update(self.vif.sma).map_err(Stop::Send)
    }
}
