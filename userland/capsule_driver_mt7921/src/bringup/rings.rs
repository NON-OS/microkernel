/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi subsystem reset, the interrupts held off, then the rings
//! (`mt7921_dma_init`): the engine stopped with a forced reset, each ring
//! reset and the receive rings armed, Bus Master on, the engine started.
//! A card that stops after this is left with its engine stopped and Bus
//! Master off.

use crate::card::Card;
use crate::dma::{wfdma, Dma};
use crate::hw::addr::TX_RING_EXT_CTRL;
use crate::hw::Regs;
use crate::mac::tokens::Tokens;
use crate::mcu::Mcu;
use crate::power::{quiet as irq_off, wfsys_reset};
use crate::say::line;
use crate::setup::{command, Mapped};
use crate::stage::Stage;

pub fn up(m: &Mapped, regs: Regs) -> Result<Card, Stage> {
    if !wfsys_reset(&regs) {
        return Err(Stage::PowerFailed);
    }
    irq_off(&regs);
    let Some(mut dma) = Dma::map(m.device_id, m.epoch) else { return Err(Stage::NoDma) };
    if !wfdma::disable(&regs, true) {
        return Err(Stage::PowerFailed);
    }
    dma.reset(&regs);
    regs.wr(TX_RING_EXT_CTRL, 0x4);
    if !command(m.device_id, m.epoch, true) {
        return Err(Stage::PowerFailed);
    }
    wfdma::enable(&regs);
    line("stage rings: WFDMA on, six rings armed");
    Ok(Card { regs, dma, mcu: Mcu::default(), tokens: Tokens::default(), woken: 0, keyed: false })
}

/// The engine stopped and Bus Master off, for a card that is not served.
pub fn quiet(card: &Card, m: &Mapped) {
    let _ = wfdma::disable(&card.regs, true);
    let _ = command(m.device_id, m.epoch, false);
    line("card quiet: WFDMA off, bus master off");
}

/// Linux's reset between firmware attempts (`mt792x_wpdma_reset`,
/// forced): the engine stopped, the subsystem reset, every ring reset and
/// re-armed, every token back, the engine on again.
pub fn rewind(card: &mut Card) -> bool {
    let ok = wfdma::disable(&card.regs, true) && wfsys_reset(&card.regs);
    card.dma.reset(&card.regs);
    card.tokens.clear();
    wfdma::enable(&card.regs);
    ok
}
