// NONOS Operating System (AGPL-3.0-or-later)

//! A key command's copies wiped: its body as soon as it is in the ring, the
//! ring slot once the engine has read past the CAM update after it and
//! `SETTLE_MS` more have passed (rtw89 keeps a command's buffer until
//! `RTW89_PCI_MULTITAG` more are taken); a slot the engine never read is
//! left alone rather than changed under it.

use nonos_wifi_core::vault::wipe;

use super::chip::Chip;
use crate::dma::DmaMem;
use crate::h2c::post::ROOM_MS;
use crate::h2c::sec_cam::{SEC_CAM_LEN, SEC_UPD};
use crate::h2c::send::h2c;
use crate::regs::Mmio;
use crate::ring::tx::SLOT;
use crate::wait::until_ms;

/// How long a key command's slot is left after the engine read past it.
pub const SETTLE_MS: u64 = 20;

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    /// One security CAM entry written; `body` wiped whatever came of it.
    pub fn sec_write(&mut self, body: &mut [u8; SEC_CAM_LEN]) -> bool {
        let ok = h2c(&self.r, &mut self.rings.ch12, &mut self.fw, SEC_UPD, body, false).is_ok();
        wipe(body);
        ok
    }

    /// Zero the `n` command slots from `first` once the engine is past them;
    /// whether it got there.
    pub(super) fn wipe_slots(&mut self, first: u32, n: u32) -> bool {
        let (r, q) = (&self.r, &mut self.rings.ch12);
        if !until_ms(ROOM_MS, || q.reclaim(r) && q.rp == q.wp) {
            return false;
        }
        until_ms(SETTLE_MS, || false);
        for i in 0..n {
            let at = q.slot_off((first + i) % q.len);
            q.mem.write(at, &[0; SLOT]);
        }
        true
    }
}
