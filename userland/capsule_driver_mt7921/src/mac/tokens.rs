/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Transmit tokens: a frame's buffer is the TXP's to read until the chip
//! names its token in a transmit-free report, so a token is held from the
//! send to that report (`mt76_token_consume`, `mt76_token_release`) and
//! its buffer is not reused before. Linux keeps 8192; this keeps 64, one
//! per frame buffer. A token no report has named in `STALE_MS` is taken
//! back, so a firmware that drops reports slows sending but never stops
//! it; Linux frees them only on a reset.

use crate::dma::layout::TOKENS;

const _: () = assert!(TOKENS == 64);
pub const STALE_MS: u64 = 3000;

pub struct Tokens {
    busy: u64,
    since: [u64; 64],
}

impl Default for Tokens {
    fn default() -> Self {
        Self { busy: 0, since: [0; 64] }
    }
}

impl Tokens {
    /// The lowest free token, now held from `now`.
    pub fn take(&mut self, now: u64) -> Option<u16> {
        let free = !self.busy;
        (free != 0).then(|| {
            let t = free.trailing_zeros();
            self.busy |= 1 << t;
            self.since[t as usize] = now;
            t as u16
        })
    }

    /// Give `t` back. A token never handed out, or out of range, is ignored:
    /// the report comes from the chip and is not trusted to name a real one.
    pub fn free(&mut self, t: u16) -> bool {
        let held = t < 64 && self.busy & (1 << t) != 0;
        if held {
            self.busy &= !(1 << t);
        }
        held
    }

    /// Take back every token held since `STALE_MS` before `now`.
    pub fn age(&mut self, now: u64) {
        for t in 0..64 {
            if self.busy & (1 << t) != 0 && now.saturating_sub(self.since[t]) >= STALE_MS {
                self.busy &= !(1 << t);
            }
        }
    }

    /// Every token back, after a reset that stopped the chip reading.
    pub fn clear(&mut self) {
        self.busy = 0;
    }

    pub fn held(&self) -> u32 {
        self.busy.count_ones()
    }
}
