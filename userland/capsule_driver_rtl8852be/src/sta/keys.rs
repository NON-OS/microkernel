// NONOS Operating System (AGPL-3.0-or-later)

//! Keys into the chip as Linux v6.12 rtw89's `rtw89_cam_sec_key_add` puts
//! them: each into a security CAM entry (`rtw89_cam_send_sec_key_cmd`),
//! then the address CAM entry pointed at them (`rtw89_cam_attach_sec_cam`,
//! `rtw89_fw_h2c_cam`); the 8852B has no DMAC table (`h2c_dctl_sec_cam` is
//! NULL). A key needs the association's entry, and is in only once the
//! engine took its commands, whose copies are wiped (`key_cmds`).

use nonos_wifi_core::softmac::ChipKey;

use super::chip::Chip;
use crate::dma::DmaMem;
use crate::h2c::cam_keys::Keys;
use crate::h2c::media::Net;
use crate::h2c::sec_cam::{sec_body, TYPE_BIP_CCMP128, TYPE_CCMP128};
use crate::regs::Mmio;

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    /// Put `k` in the chip; false, and no slot left pointed at it, if the
    /// engine did not take its commands.
    pub fn put_key(&mut self, k: &ChipKey<'_>) -> bool {
        let one = |pairwise, id, kind, key| Keys::slot(pairwise, id).map(|s| (s, id, kind, key));
        let keys = match *k {
            ChipKey::Pairwise(tk) => [one(true, 0, TYPE_CCMP128, tk), None],
            ChipKey::Group { id, gtk, igtk } => {
                let i = igtk.and_then(|(i, key)| one(false, i, TYPE_BIP_CCMP128, key));
                [one(false, id, TYPE_CCMP128, gtk), i]
            }
        };
        if self.vif.net != Net::Infra || keys[0].is_none() {
            return false;
        }
        let (first, before) = (self.rings.ch12.wp, self.vif.keys);
        let (mut sent, mut ok) = (0, true);
        for (slot, id, kind, key) in keys.into_iter().flatten() {
            ok &= self.sec_write(&mut sec_body(slot as u8, kind, key));
            self.vif.keys.set(slot, id);
            sent += 1;
        }
        let ok = (ok && self.cam_update(self.vif.sma).is_ok()) & self.wipe_slots(first, sent);
        if !ok {
            self.vif.keys = before;
        }
        ok
    }
}
