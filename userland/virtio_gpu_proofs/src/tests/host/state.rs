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
//! The host's state and how it is cued to go wrong.
//!
//! A scanout shows its resource's pixels as they are now, not as of the
//! last flush: stricter than a host that only repaints on flush, so a
//! picture the model does not show is one no host would. A resource's
//! pixels start as POISON, as nothing says a host zeroes them.

use std::collections::BTreeMap;

use crate::constants::VG_MAX_SCANOUTS;

pub const POISON: u32 = 0xDEAD_BEEF;

pub struct Res {
    pub w: u32,
    pub h: u32,
    pub px: Vec<u32>,
    pub backing: Option<(u64, u32)>,
}

/// One command as the host saw it, and whether it was carried out.
#[derive(Clone, Copy, Debug)]
pub struct Cmd {
    pub kind: u32,
    pub landed: bool,
}

pub struct Host {
    pub res: BTreeMap<u32, Res>,
    /// Each scanout's resource; 0 is detached.
    pub scanouts: [u32; VG_MAX_SCANOUTS],
    pub log: Vec<Cmd>,
    /// Ring heads taken and used entries written.
    pub seen: u16,
    pub used: u16,
    /// From command `dead_at` the host takes commands and answers none, for
    /// `dead_for` of them (0: for good); at `die_at` the driver is killed;
    /// command `refuse_at` is answered refused and not carried out.
    pub dead_at: Option<u32>,
    pub dead_for: u32,
    pub die_at: Option<u32>,
    pub refuse_at: Option<u32>,
    /// What GET_DISPLAY_INFO reports per scanout: (width, height), 0 off.
    pub displays: [(u32, u32); VG_MAX_SCANOUTS],
}

impl Host {
    pub fn steps(&self) -> u32 {
        self.log.len() as u32
    }

    pub fn dead(&self, step: u32) -> bool {
        self.dead_at.is_some_and(|k| step >= k && (self.dead_for == 0 || step < k + self.dead_for))
    }

    /// What scanout `s` shows, if anything.
    pub fn shown(&self, s: usize) -> Option<&[u32]> {
        self.res.get(&self.scanouts[s]).map(|r| &r.px[..])
    }
}
