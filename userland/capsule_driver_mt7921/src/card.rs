/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A brought-up card: its registers, its rings, its MCU channel and its
//! transmit tokens, which every step after the firmware works through.

use crate::dma::Dma;
use crate::hw::Regs;
use crate::mac::tokens::Tokens;
use crate::mcu::cmd::Cmd;
use crate::mcu::{Mcu, McuError, Reply};

pub struct Card {
    pub regs: Regs,
    pub dma: Dma,
    pub mcu: Mcu,
    pub tokens: Tokens,
    /// Times the card was found in its firmware's hands (`power::wake`).
    pub woken: u32,
}

impl Card {
    /// A command, once the card is the driver's (`awake`).
    pub fn send(&mut self, cmd: Cmd, body: &[u8], wait: bool) -> Result<Reply, McuError> {
        if !self.awake() {
            return Err(McuError::Asleep);
        }
        self.mcu.send(&self.regs, &mut self.dma, cmd, body, wait)
    }

    /// `send`, with only whether it went through.
    pub fn cmd(&mut self, cmd: Cmd, body: &[u8], wait: bool) -> bool {
        self.send(cmd, body, wait).is_ok()
    }

    /// Read the event rings, noting what came unasked (ROC grants).
    pub fn pump(&mut self) {
        self.mcu.pump(&self.regs, &mut self.dma);
    }

    /// Whether the card still answers: a card gone from the bus reads all
    /// ones, and the WFDMA's configuration never does.
    pub fn alive(&self) -> bool {
        self.regs.rr(crate::hw::addr::GLO_CFG) != u32::MAX
    }
}
