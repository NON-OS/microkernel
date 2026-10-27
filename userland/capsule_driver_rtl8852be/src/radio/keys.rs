// NONOS Operating System (AGPL-3.0-or-later)

//! A key into the chip (`sta::keys`), counted when a command failed, and
//! one line saying which key went in or was refused, never the key.

use super::counts::bump;
use super::Station;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::status::Line;
use nonos_wifi_core::softmac::ChipKey;

impl<R: Mmio, M: DmaMem> Station<R, M> {
    pub fn key(&mut self, k: &ChipKey<'_>) -> bool {
        let ok = self.chip.put_key(k);
        if !ok {
            bump(&mut self.counts.cmd_fail);
        }
        let what: &[u8] = match k {
            ChipKey::Pairwise(_) => b"[rtl8852be] pairwise key",
            ChipKey::Group { .. } => b"[rtl8852be] group key",
        };
        let said: &[u8] = if ok { b" in the chip" } else { b" refused by the firmware" };
        Line::new(what).text(said).emit();
        ok
    }
}
