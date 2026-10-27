/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What the radio counts, and the five words the status op carries for a
//! person bringing up a card with no serial console: the chip, the
//! channel grants and those that never came, the frames refused at
//! receive, and the commands the firmware did not accept.

use super::Mt7921;

#[derive(Clone, Copy, Default)]
pub struct Counts {
    pub tx: u32,
    pub tx_drop: u32,
    pub rx: u32,
    pub rx_bad: u32,
    pub grants: u32,
    pub missed: u32,
    pub refused: u32,
}

impl Mt7921 {
    /// Count a command sequence the firmware did not take.
    pub(super) fn note(&mut self, ok: bool) {
        if !ok {
            self.count.refused = self.count.refused.wrapping_add(1);
        }
    }

    pub(super) fn diag_words(&self) -> [u32; 5] {
        let c = self.count;
        [self.chip.number(), c.grants, c.missed, c.rx_bad, c.refused]
    }

    /// The chip's upkeep between requests: the card taken back if its
    /// firmware holds it (`power::wake`), the event rings read (ROC grants
    /// and the firmware's own notices), a channel request renewed, stale
    /// tokens taken back. A card that reads all ones is dead from
    /// then on, and the station's link reads down.
    pub(super) fn upkeep(&mut self, now: u64) {
        if self.dead {
            return;
        }
        if !self.card.alive() {
            self.dead = true;
            crate::say::line("card stopped answering: radio off");
            return;
        }
        if !self.card.awake() {
            return;
        }
        self.card.pump();
        self.renew(now);
        self.card.tokens.age(now);
    }
}
