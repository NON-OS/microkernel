/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Sending one frame the station built (`mt7921e_tx_prepare_skb`): a
//! token's buffer takes the frame, ring 0 takes its TXD and TXP. A frame
//! to the access point goes under its WTBL entry, any other under the
//! interface's own (as mt76 sends one with no station). Management and
//! group frames go at the band's lowest rate, the association's data at
//! its fastest basic rate.

use nonos_wifi_core::dot11::header::fc_type;

use super::Mt7921;
use crate::clock::now_ms;
use crate::dma::layout::TOKEN_BUF;
use crate::mac::rate;
use crate::mac::txd::{txwi, TxSpec};
use crate::vif::{AP_WCID, OWN_WCID};

impl Mt7921 {
    pub(super) fn send_frame(&mut self, mpdu: &[u8]) -> bool {
        if self.dead || mpdu.len() < 24 || mpdu.len() > TOKEN_BUF {
            return false;
        }
        let now = now_ms();
        self.card.tokens.age(now);
        if self.card.tokens.held() as usize == crate::dma::layout::TOKENS {
            self.reap();
        }
        let Some(token) = self.card.tokens.take(now) else {
            self.count.tx_drop = self.count.tx_drop.wrapping_add(1);
            return false;
        };
        let to_ap = self.peer.is_some_and(|p| mpdu[4..10] == p.addr);
        let data = fc_type(u16::from_le_bytes([mpdu[0], mpdu[1]])) == 2;
        let ghz5 = self.ghz5();
        let fast = self.joined.filter(|_| data && to_ap).map(|j| j.1);
        let spec = TxSpec {
            wcid: if to_ap { AP_WCID } else { OWN_WCID },
            token,
            frame_dev: self.card.dma.tokens.dev(token as usize * TOKEN_BUF),
            rate: rate::value(fast.unwrap_or(rate::lowest(ghz5)), ghz5),
        };
        let c = &mut self.card;
        c.dma.tokens.write(token as usize * TOKEN_BUF, mpdu);
        c.dma.tx_data.reclaim(&c.regs);
        let queued = txwi(mpdu, &spec).and_then(|w| c.dma.tx_data.push(&w)).is_some();
        if !queued {
            c.tokens.free(token);
            self.count.tx_drop = self.count.tx_drop.wrapping_add(1);
            return false;
        }
        c.dma.tx_data.kick(&c.regs);
        self.count.tx = self.count.tx.wrapping_add(1);
        true
    }
}
