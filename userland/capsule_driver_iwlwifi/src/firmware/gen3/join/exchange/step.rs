// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Every frame the inbox holds, fed to the MLME in order, and its replies
//! sent. When the association response moves the MLME to the four-way
//! handshake, the firmware is told before any EAPOL frame is answered.

use nonos_wifi_core::mlme::{Mlme, MlmeFailure, MlmeState};

use super::super::super::region::{Clock, Region};
use super::super::bss::Bss;
use super::super::fw::Fw;
use super::air::Air;
use super::assoc::association;
use super::end::ExchangeEnd;
use crate::regs::Mmio;

/// What the frames taken did to the exchange.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Nothing moved: no reply sent, no state changed.
    Quiet,
    /// A reply went out, or the state moved on.
    Moved,
    /// The port is open.
    Connected,
}

pub fn take_frames<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    bss: &mut Bss,
    mlme: &mut Mlme,
    air: &mut Air,
) -> Result<Step, ExchangeEnd> {
    let mut step = Step::Quiet;
    while let Some(rx) = fw.inbox.frames.pop_front() {
        let before = mlme.state();
        let reply = air.feed(mlme, &rx);
        let now = mlme.state();
        if before == MlmeState::Associating && now == MlmeState::FourWay {
            let (aid, cap) = association(&rx.frame);
            bss.associated(fw, aid, cap).map_err(|c| ExchangeEnd::Firmware(Some(c)))?;
        }
        match (reply, now) {
            (_, MlmeState::Failed) => {
                return Err(ExchangeEnd::Refused(
                    mlme.failure().unwrap_or(MlmeFailure::AssocRejected(0)),
                ))
            }
            (Some(tx), _) => {
                air.send(fw, bss, &tx);
                air.pending = tx;
                step = Step::Moved;
            }
            (None, _) if now != before => {
                air.pending.clear();
                step = Step::Moved;
            }
            _ => {}
        }
        if now == MlmeState::Connected {
            return Ok(Step::Connected);
        }
    }
    Ok(step)
}
