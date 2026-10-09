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

/*
 * Which tiers the Qwen step offers, and which row it starts on. Rows are
 * "None for now", then the tiers that fit this machine, smallest first;
 * the rest are not listed, and one line says how many there are.
 *
 * Whether a tier fits is the model fetcher's rule (need.rs, `tier_fits`),
 * the one the store's cards and `qwen get` hold a tier to, and who the
 * tier is for says where its model is kept:
 *
 * - Installed: the Mode step chose to install, so the model is kept on
 *   that disk and only its run is held in memory.
 * - Session: an amnesic boot from a NONOS stick. The kernel holds this
 *   session's volume in memory, gone at power off, so the model's file is
 *   held there and read again to run it.
 * - NoDisk: no NONOS disk at all, so nothing can hold a model, not even in
 *   memory. Nothing is offered and the step starts on none.
 *
 * The step starts on the default the dock and the Terminal run when no
 * tier is chosen (`default_tier::resolve`): the stick tier, which installs
 * offline, when it fits, else the largest tier that fits. Nothing that does
 * not fit is a row, so with nothing offered it starts on none.
 *
 * Pure, so setup_layout_proofs holds the rules over the real pins.
 */

use alloc::vec::Vec;

use super::default_tier::resolve;
use super::need::{tier_fits, Room};

/* Who the chosen tier is for. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum For {
    Installed,
    Session,
    NoDisk,
}

/*
 * The indices of `tiers` (smallest first) that fit `memory` for `who`, in
 * that order. Memory the kernel would not report fits nothing.
 */
pub fn offered(tiers: &[(&[u8], u64)], memory: Option<u64>, who: For) -> Vec<u8> {
    let room = match who {
        For::Installed => Room::Disk,
        For::Session => Room::Memory,
        For::NoDisk => return Vec::new(),
    };
    let Some(total) = memory else { return Vec::new() };
    let fit = |(tier, bytes): &(&[u8], u64)| {
        let tier = core::str::from_utf8(tier).unwrap_or("");
        tier_fits(room, tier, *bytes, total) == Some(true)
    };
    tiers.iter().enumerate().filter(|(_, t)| fit(t)).map(|(i, _)| i as u8).collect()
}

/*
 * The row the step starts on: 0 is none, i + 1 is `tiers[offered[i]]`. The
 * default for this machine, as the dock and the Terminal pick it, when it
 * is offered.
 */
pub fn default_row(tiers: &[(&[u8], u64)], offered: &[u8], memory: Option<u64>, who: For) -> u8 {
    let room = match who {
        For::Installed => Room::Disk,
        For::Session => Room::Memory,
        For::NoDisk => return 0,
    };
    let (tier, _) = resolve(b"", memory, room);
    let word = |i: &u8| tiers.get(*i as usize).map_or(&b""[..], |t| t.0);
    offered.iter().position(|i| word(i) == tier.as_bytes()).map_or(0, |at| at as u8 + 1)
}
