/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Receiving: a frame kept aside by a send first, then the data ring. A
//! beacon or probe response met on the way has its basic rates noted. A
//! frame goes up with the chip's word on whether it decrypted it.

use nonos_wifi_core::softmac::RxInfo;

use super::Mt7921;

impl Mt7921 {
    pub(super) fn recv_frame(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        if self.dead {
            return None;
        }
        let (len, rssi_dbm, decrypted) = self.next_kept(out).or_else(|| self.next_frame(out))?;
        self.learn(&out[..len]);
        Some(RxInfo { len, rssi_dbm, decrypted })
    }

    fn next_kept(&mut self, out: &mut [u8]) -> Option<(usize, i8, bool)> {
        while let Some((f, rssi_dbm, dec)) = self.inbox.pop_front() {
            if f.len() <= out.len() {
                out[..f.len()].copy_from_slice(&f);
                return Some((f.len(), rssi_dbm, dec));
            }
        }
        None
    }
}
