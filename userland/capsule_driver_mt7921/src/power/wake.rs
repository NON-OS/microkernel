/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The card kept the driver's. mt76 hands the card to its firmware when
//! idle and takes it back before touching it (`mt76_connac_pm_wake`,
//! `mt792xe_mcu_drv_pmctrl`), then re-arms the WFDMA if the doze lost it
//! (`mt792x_wpdma_reinit_cond`: `MT_WFDMA_NEED_REINIT` found clear). This
//! driver never hands the card over once it runs, but a laptop's link power
//! management, or a firmware that reset itself, can leave it in the
//! firmware's hands. So before every command and every frame sent, and on
//! every service pass, ownership is read (`OWN_SYNC` set is the
//! firmware's), taken back when lost, and the rings re-armed when the WFDMA
//! forgot them. A line says so on the first loss and then on every
//! power-of-two count, so a card that flaps cannot flood the log.

use super::own::drv_own;
use super::quiet;
use crate::card::Card;
use crate::dma::wfdma;
use crate::hw::addr::*;
use crate::say::Line;

impl Card {
    /// Whether the driver owns the card now, taking it back when it must.
    pub fn awake(&mut self) -> bool {
        let lp = self.regs.rr(CONN_ON_LPCTL);
        if lp == u32::MAX || lp & LPCR_HOST_OWN_SYNC == 0 {
            return lp != u32::MAX;
        }
        self.woken = self.woken.wrapping_add(1);
        let say = self.woken.is_power_of_two();
        if !drv_own(&self.regs) {
            let l = Line::new("ownership: the firmware holds the card and would not give it back");
            if say {
                l.say();
            }
            return false;
        }
        let lost = self.regs.rr(WFDMA_DUMMY_CR) & WFDMA_NEED_REINIT == 0;
        if lost {
            self.rearm();
        }
        if say {
            let l = Line::new("ownership: taken back from the firmware, time ");
            l.dec(self.woken.into()).text(if lost { "; WFDMA re-armed" } else { "" }).say();
        }
        true
    }

    /// `mt792x_wpdma_reset`, unforced: interrupts held off, the engine
    /// stopped, every ring reset and re-armed, every token back (the
    /// frames in flight are gone with the rings), the engine on again.
    fn rearm(&mut self) {
        quiet(&self.regs);
        let _ = wfdma::disable(&self.regs, false);
        self.dma.reset(&self.regs);
        self.tokens.clear();
        wfdma::enable(&self.regs);
    }
}
