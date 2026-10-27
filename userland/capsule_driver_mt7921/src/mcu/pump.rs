/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Reading the two event rings. A reply is handed to whoever waits for
//! it; an unasked event is noted, and the one the driver acts on is the
//! ROC grant (`mt7921_mcu_uni_roc_event`): its TLV sits four bytes into
//! the body, `struct mt7921_roc_grant_tlv`, tag `UNI_EVENT_ROC_GRANT` (0).

use super::cmd::Cmd;
use super::event::Event;
use super::reply::{matches, Reply};
use super::{Grant, Mcu, McuError};
use crate::dma::layout::RX_BUF;
use crate::dma::{Dma, Rx};
use crate::hw::Regs;

/// `MCU_UNI_EVENT_ROC`.
const UNI_EVENT_ROC: u8 = 0x27;

impl Mcu {
    /// Drain both event rings, returning the reply to `want` if it came.
    pub(super) fn pump_for(
        &mut self,
        regs: &Regs,
        dma: &mut Dma,
        want: Option<(u8, Cmd)>,
    ) -> Option<Result<Reply, McuError>> {
        let mut buf = [0u8; RX_BUF];
        for ring in [&mut dma.rx_mcu, &mut dma.rx_wa] {
            while let Some(rx) = ring.take(regs, &mut buf) {
                let Rx::Frame(n) = rx else { continue };
                let Some(ev) = Event::parse(&buf[..n]) else { continue };
                if ev.unsolicited() {
                    self.note(&ev);
                } else if let Some(r) = want.and_then(|(seq, cmd)| matches(seq, cmd, &ev)) {
                    return Some(r);
                }
            }
        }
        None
    }

    /// Drain both event rings, noting what came unasked.
    pub fn pump(&mut self, regs: &Regs, dma: &mut Dma) {
        let _ = self.pump_for(regs, dma, None);
    }

    /// An unasked event that came on the data ring, noted as `pump` does.
    pub fn note_unasked(&mut self, b: &[u8]) {
        if let Some(ev) = Event::parse(b).filter(|e| e.unsolicited()) {
            self.note(&ev);
        }
    }

    fn note(&mut self, ev: &Event) {
        self.events = self.events.wrapping_add(1);
        let b = ev.body();
        if ev.uni_unsolicited(UNI_EVENT_ROC) && b.len() >= 12 && b[4] == 0 && b[5] == 0 {
            self.grant = Some(Grant { token: b[9], channel: b[11] });
        }
    }
}
