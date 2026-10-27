/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Receiving: a frame kept aside by a send first, then the data ring. A
//! beacon or probe response met on the way has its basic rates noted.
//! Every frame goes up as received: the chip holds no key, so it
//! decrypts nothing and the station checks every protected frame itself.

use nonos_wifi_core::softmac::RxInfo;

use super::Mt7921;

impl Mt7921 {
    pub(super) fn recv_frame(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        if self.dead {
            return None;
        }
        let (len, rssi_dbm) = self.next_kept(out).or_else(|| self.next_frame(out))?;
        self.learn(&out[..len]);
        Some(RxInfo { len, rssi_dbm, decrypted: false })
    }

    fn next_kept(&mut self, out: &mut [u8]) -> Option<(usize, i8)> {
        while let Some((f, rssi_dbm)) = self.inbox.pop_front() {
            if f.len() <= out.len() {
                out[..f.len()].copy_from_slice(&f);
                return Some((f.len(), rssi_dbm));
            }
        }
        None
    }
}
