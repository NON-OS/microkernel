// NONOS Operating System (AGPL-3.0-or-later)

//! The station's BSS and association turned into rtw89's steps, and the
//! diagnostic words. An associated station scanning off channel keeps its
//! AP (rtw89's software scan does), and coming home to it only ends the
//! scan; a new BSS while associated leaves the old one first.

use super::counts::bump;
use super::Station;
use crate::dma::DmaMem;
use crate::h2c::media::Net;
use crate::regs::Mmio;
use nonos_wifi_core::softmac::Association;

impl<R: Mmio, M: DmaMem> Station<R, M> {
    /// `set_bss`.
    pub fn point(&mut self, bssid: Option<[u8; 6]>) {
        let joined = self.chip.vif.net == Net::Infra;
        let ok = match bssid {
            None => self.chip.scan_start().is_ok(),
            Some(b) if joined && b == self.chip.vif.bssid => {
                self.chip.scan_end();
                true
            }
            Some(b) => (!joined || self.chip.leave().is_ok()) && self.chip.join(b).is_ok(),
        };
        if !ok {
            bump(&mut self.counts.cmd_fail);
        }
    }

    /// `associated`.
    pub fn settle(&mut self, assoc: Option<&Association>) {
        let ok = match assoc {
            Some(a) => {
                let n = a.n_rates.min(16);
                self.chip.vif.rates[..n].copy_from_slice(&a.rates[..n]);
                self.chip.vif.n_rates = n;
                self.chip.associated(a.aid, &a.rates[..n], a.beacon_int).is_ok()
            }
            None => self.chip.leave().is_ok(),
        };
        if !ok {
            bump(&mut self.counts.cmd_fail);
        }
    }

    /// `diag`: command and tune failures, ring-full and dropped counts,
    /// bad CRCs, DACK timeouts and the IQK fail bits, the channel.
    pub fn words(&self) -> [u32; 5] {
        let c = &self.counts;
        let rfk = &self.chip.rfk;
        let iqk_fail = rfk.iqk.fail.iter().flatten().fold(0u32, |m, &f| m << 1 | f as u32);
        [
            (c.cmd_fail as u32) << 16 | c.tune_fail as u32,
            (c.tx_full as u32) << 16 | c.rx_drop as u32,
            c.rx_bad_crc as u32,
            (rfk.dack.timed_out() as u32) << 8 | iqk_fail,
            self.chip.chan.map_or(0, |ch| ch.ch as u32 | (ch.five as u32) << 8),
        ]
    }
}
