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

//! The takeover model's whole state (L79), one value per world. Paths by
//! index: 0 G1 and 1 G2, covering claims; 2 B, bootfb's thread that claims,
//! maps, stores and gives back; 3 its sibling thread; 4 E, bootfb's exit;
//! 5 K, a kernel drawer; 6 the exit that orphans G1's claim being made.

use super::starts::Cfg;
use super::world::{Entry, Page};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct World {
    /// bootfb's claim entry, and G1's and G2's, each as a stage code.
    pub fb: Option<Entry>,
    pub gpu: [Option<u8>; 2],
    /// bootfb's grant of each round recorded, and the page it maps.
    pub grant: [bool; 2],
    pub page: [Page; 2],
    /// HELD, GONE, TAKERS and DRAWS as the kernel keeps them.
    pub held: bool,
    pub gone: bool,
    pub takers: u8,
    pub draws: u8,
    pub pc: [u8; 7],
    pub g: [Taker; 2],
    /// B's round, and its give_back's epoch and drained rounds.
    pub round: u8,
    pub gb: (u8, u8),
    /// B's first grant recorded; its sibling's unmap counted inside.
    pub mapped: bool,
    pub inside: bool,
    /// The epoch bootfb's exit counted itself a release of; its threads off
    /// a CPU for good; its tables freed.
    pub exit: Option<u8>,
    pub dead: bool,
    pub freed: bool,
    /// A covering claim was made ready at some point.
    pub ever_in: bool,
    pub cfg: Cfg,
    pub events: u16,
    pub bad: u16,
}

/// A covering claim's own: the stage its read last took back, the epoch its
/// take-back retired, the rounds it drained, its admit's answer, its wipe
/// record written, and whether a bootfb claim was made ready since its
/// takeover last looked.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(super) struct Taker {
    pub seen: u8,
    pub epoch: u8,
    pub drained: u8,
    pub ok: bool,
    pub wiped: bool,
    pub late: bool,
}
