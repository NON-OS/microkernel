// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The event state the capsule keeps between SCIs: the fixed events it
//! serves, the GPE blocks, and which runtime events are armed.

use super::error::EventsError;
use super::fadt::Fadt;
use super::gpe_block::GpeBlock;
use super::handlers::Handler;
use super::pm1::{Pm1, GBL, PWRBTN, SLPBTN};
use super::policy::Policy;
use super::storm::Storms;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Events {
    pub pm1: Pm1,
    pub blocks: Vec<GpeBlock>,
    pub handlers: BTreeMap<u16, Handler>,
    /// Events enabled now: in a block, and no device's wake event.
    pub armed: BTreeSet<u16>,
    /// Served by the capsule itself (the EC's, 12.6), as edge events.
    pub native: BTreeSet<u16>,
    /// PM1 enables: the Global Lock's release always, buttons by policy. The
    /// timer and RTC alarm are never enabled; the RTC is a wake source.
    pub fixed: u16,
    pub storms: Storms,
}

impl Events {
    pub fn new(
        f: &Fadt,
        handlers: BTreeMap<u16, Handler>,
        wake: &BTreeSet<u16>,
        policy: Policy,
    ) -> Result<Self, EventsError> {
        if f.hw_reduced {
            return Err(EventsError::HwReduced);
        }
        let pm1 = Pm1 { a: f.pm1a_evt.ok_or(EventsError::NoPm1)?, b: f.pm1b_evt };
        let mut blocks: Vec<GpeBlock> = f.gpe0.map(|b| GpeBlock::new(b, 0)).into_iter().collect();
        if let Some(b) = f.gpe1.map(|b| GpeBlock::new(b, u16::from(f.gpe1_base))) {
            let (lo, hi) = b.range();
            if blocks.iter().any(|g| g.range().0 < hi && lo < g.range().1) {
                return Err(EventsError::Overlap);
            }
            blocks.push(b);
        }
        let armed = handlers
            .keys()
            .copied()
            .filter(|n| !wake.contains(n) && blocks.iter().any(|b| b.holds(*n)))
            .collect();
        let buttons = [(policy.power_button, PWRBTN), (policy.sleep_button, SLPBTN)];
        let fixed = buttons.iter().filter(|(on, _)| *on).fold(GBL, |acc, (_, bit)| acc | bit);
        let (native, storms) = (BTreeSet::new(), Storms::default());
        Ok(Events { pm1, blocks, handlers, armed, native, fixed, storms })
    }
}
