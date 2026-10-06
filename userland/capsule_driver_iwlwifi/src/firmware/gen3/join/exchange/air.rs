// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The air side of the exchange: what is pending, the sequence number of the
//! EAPOL replies, the counts, and a frame the MLME produced sent on the queue
//! its type belongs on.

use alloc::vec::Vec;

use nonos_wifi_core::dot11::header::TYPE_MGMT;

use super::super::super::region::{Clock, Region};
use super::super::super::station::rates::rate_n_flags;
use super::super::bss::Bss;
use super::super::fw::{Fw, Queue};
use super::progress::Progress;
use crate::regs::Mmio;

pub fn frame_type(frame: &[u8]) -> Option<u8> {
    frame.first().map(|b| (b >> 2) & 0x3)
}

pub struct Air {
    pub pending: Vec<u8>,
    pub seq: u16,
    pub p: Progress,
}

impl Air {
    // Send a frame the MLME produced on the queue its type belongs on.
    pub fn send<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        bss: &Bss,
        frame: &[u8],
    ) {
        let which = if frame_type(frame) == Some(TYPE_MGMT) { Queue::Mgmt } else { Queue::Data };
        let rate = rate_n_flags(bss.target.rates.mgmt, bss.tx_ant);
        if fw.send(which, frame, rate) {
            self.p.sent = self.p.sent.saturating_add(1);
        } else {
            self.p.refused = self.p.refused.saturating_add(1);
        }
    }
}
