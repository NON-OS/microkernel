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
//! The controllers served, and where each host slot is.

use super::hc::Hc;
use crate::devices::table::DEVS;

/// Controllers served at most: an ICH's six UHCI, with room.
pub const MAX_HC: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bind {
    Free,
    /// Enabled, not yet addressed on any controller.
    Enabled,
    /// Addressed on controller `k`.
    On(usize),
}

pub struct Front<H: Hc> {
    pub hcs: [Option<H>; MAX_HC],
    pub bound: [Bind; DEVS],
}

impl<H: Hc> Front<H> {
    pub fn new() -> Self {
        Front { hcs: core::array::from_fn(|_| None), bound: [Bind::Free; DEVS] }
    }

    /// The first free place for a controller, taken by `hc`.
    pub fn add(&mut self, hc: H) -> bool {
        let Some(place) = self.hcs.iter_mut().find(|h| h.is_none()) else { return false };
        *place = Some(hc);
        true
    }

    /// Host port `port` as (controller, its own port).
    pub fn local(&self, port: u8) -> Option<(usize, u8)> {
        let mut base = 0u8;
        for (k, hc) in self.hcs.iter().enumerate() {
            let Some(hc) = hc else { continue };
            if (base + 1..=base.saturating_add(hc.ports())).contains(&port) {
                return Some((k, port - base));
            }
            base = base.saturating_add(hc.ports());
        }
        None
    }

    /// Controller `k`'s first host port, less one.
    pub fn base(&self, k: usize) -> u8 {
        self.hcs[..k].iter().flatten().fold(0u8, |b, h| b.saturating_add(h.ports()))
    }
}

impl<H: Hc> Default for Front<H> {
    fn default() -> Self {
        Self::new()
    }
}
