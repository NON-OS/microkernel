/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Receiving: a frame kept aside by a send first, then the data ring.
//! Every frame goes up as received: the chip holds no key, so it
//! decrypts nothing and the station checks every protected frame itself.

use nonos_wifi_core::softmac::RxInfo;

use super::Mt7921;

impl Mt7921 {
    pub(super) fn recv_frame(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        if self.dead {
            return None;
        }
        while let Some((f, rssi_dbm)) = self.inbox.pop_front() {
            if f.len() <= out.len() {
                out[..f.len()].copy_from_slice(&f);
                return Some(RxInfo { len: f.len(), rssi_dbm, decrypted: false });
            }
        }
        let (len, rssi_dbm) = self.next_frame(out)?;
        Some(RxInfo { len, rssi_dbm, decrypted: false })
    }
}
