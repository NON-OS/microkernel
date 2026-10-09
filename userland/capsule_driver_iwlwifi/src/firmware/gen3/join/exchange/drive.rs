// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The exchange's loop: from the authentication frame to a connected MLME,
//! or to its end.

use alloc::vec::Vec;

use nonos_wifi_core::mlme::Mlme;

use super::super::super::region::{Clock, Region};
use super::super::super::station::session::Session;
use super::super::bss::Bss;
use super::super::fw::Fw;
use super::air::Air;
use super::end::ExchangeEnd;
use super::limits::{EXCHANGE_MS, IDLE_TRIES, RETX_MS, SILENT_WAITS_MAX};
use super::step::{take_frames, Step};
use crate::regs::Mmio;

pub fn run<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    bss: &mut Bss,
    mlme: &mut Mlme,
    first: Vec<u8>,
    air: &mut Air,
) -> Result<(), ExchangeEnd> {
    air.send(fw, bss, &first);
    air.pending = first;
    let start = fw.clock.now_ms();
    let (mut sent_at, mut idle, mut silent) = (start, 0u32, 0u32);
    loop {
        match take_frames(fw, bss, mlme, air)? {
            Step::Connected => return Ok(()),
            Step::Moved => (sent_at, idle) = (fw.clock.now_ms(), 0),
            Step::Quiet => {}
        }
        if let Some(Session::Ended) | Some(Session::Refused) = fw.inbox.session.take() {
            bss.session = false;
            bss.protect(fw).map_err(|c| ExchangeEnd::Firmware(Some(c)))?;
        }
        let now = fw.clock.now_ms();
        if now.saturating_sub(start) >= EXCHANGE_MS || silent >= SILENT_WAITS_MAX {
            return Err(ExchangeEnd::TimedOut);
        }
        // The resend is due on the clock, however many unrelated frames
        // arrived meanwhile: a busy channel never goes quiet for `RETX_MS`.
        let since = now.saturating_sub(sent_at);
        if since >= u64::from(RETX_MS) {
            idle += 1;
            if idle > IDLE_TRIES {
                return Err(ExchangeEnd::TimedOut);
            }
            if !air.pending.is_empty() {
                let again = air.pending.clone();
                air.send(fw, bss, &again);
            }
            sent_at = now;
            continue;
        }
        match fw.wait(RETX_MS - since as u32) {
            Ok(true) => {}
            Ok(false) => silent += 1,
            Err(_) => return Err(ExchangeEnd::Firmware(None)),
        }
    }
}
