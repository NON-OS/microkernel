// NONOS Operating System (AGPL-3.0-or-later)

//! The station interface in the firmware and MAC: `rtw89_mac_vif_init`
//! (Linux v6.12 rtw89 `mac.c`), run once after bring-up, and the CAM
//! update rtw89 sends whenever the station's or the AP's address changes
//! (`rtw89_fw_h2c_cam`). The 8852B has no default DMAC table command.

use super::chip::Chip;
use crate::dma::DmaMem;
use crate::h2c::cam::{cam, cam_body};
use crate::h2c::cctl::{default_tbl, send};
use crate::h2c::media::{join_info, role_maintain, Net, Upd};
use crate::h2c::send::SendFail;
use crate::mac::macid::{tbl_init, unpause};
use crate::mac::port::port_update;
use crate::regs::Mmio;

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    /// The CAM entry for `sma` (the scan's or the station's address).
    pub fn cam_update(&mut self, sma: [u8; 6]) -> Result<(), SendFail> {
        let v = &self.vif;
        let body = cam_body(&sma, &v.bssid, v.net, v.aid, v.keys);
        cam(&self.r, &mut self.rings.ch12, &mut self.fw, &body)
    }

    pub fn vif_init(&mut self) -> Result<(), SendFail> {
        let (r, q, fw) = (&self.r, &mut self.rings.ch12, &mut self.fw);
        port_update(r, Net::NoLink, 0);
        tbl_init(r);
        unpause(r, q, fw)?;
        role_maintain(r, q, fw, Upd::Create)?;
        join_info(r, q, fw, true, Net::NoLink)?;
        self.cam_update(self.vif.sma)?;
        send(&self.r, &mut self.rings.ch12, &mut self.fw, &default_tbl())
    }
}
