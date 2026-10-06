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
 * Which tier Qwen runs when it is opened without one named: the dock and
 * the Launchpad, the Terminal's `qwen`, and the row setup's Qwen step
 * starts on all take it from here, so none of them starts a tier the
 * person never chose in silence. The rule, in order:
 *
 * 1. The tier the person chose, at setup or in Settings.
 * 2. Else the stick tier (`STICK_TIER`), which the release stick carries
 *    and which installs offline, when it fits this machine.
 * 3. Else the largest tier that fits this machine's memory.
 * 4. Else the smallest tier, which then says it does not fit.
 *
 * Fitting is need.rs's rule, the one the store, setup and the fetcher hold
 * a tier to; memory the kernel would not report counts the stick tier as
 * fitting, since it is the smallest Qwen3. Anything but the person's choice
 * is a default, and each caller says so: the Terminal in a line, the dock
 * in a toast. Pure, so
 * model_fetch_proofs, setup_layout_proofs and terminal_line_proofs hold it.
 */

use super::need::{tier_fits, Room, STICK_TIER};

/*
 * Each tier's pinned files, summed, smallest first: what a default is
 * weighed by. model_fetch_proofs holds this table equal to the pins.
 */
#[rustfmt::skip]
pub const WEIGHTS: &[(&str, u64)] = &[
    ("small", 491_400_032),
    ("qwen3-0.6b", 639_446_688),
    ("medium", 1_117_320_736),
    ("coder-1.5b", 1_117_320_768),
    ("qwen3-1.7b", 1_834_426_016),
    ("large", 2_104_932_768),
    ("qwen3-4b", 2_497_280_256),
    ("xlarge", 4_683_073_632),
    ("coder-7b", 4_683_073_664),
    ("qwen3-8b", 5_027_783_488),
    ("coder-14b", 8_988_110_400),
    ("xxl", 8_988_110_496),
    ("qwen3-14b", 9_001_752_960),
    ("qwen3-30b-a3b", 18_556_685_824),
    ("qwen3-32b", 19_762_149_024),
    ("coder-32b", 19_851_336_128),
    ("max", 19_851_336_384),
];

/* Why a tier is the one that runs. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    Chosen,
    Stick,
    Largest,
    Smallest,
}

/*
 * The tier to run for `chosen`, the tier word the policy store holds (empty
 * for none; anything after the word is not part of it), on a machine of
 * `memory` bytes whose models are kept in `room`.
 */
pub fn resolve(chosen: &[u8], memory: Option<u64>, room: Room) -> (&'static str, Source) {
    let word_byte = |c: &u8| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'.' || *c == b'-';
    let word = &chosen[..chosen.iter().position(|c| !word_byte(c)).unwrap_or(chosen.len())];
    if let Some(&(t, _)) = WEIGHTS.iter().find(|(t, _)| t.as_bytes() == word && !word.is_empty()) {
        return (t, Source::Chosen);
    }
    let fits = |(t, b): &(&str, u64)| memory.and_then(|m| tier_fits(room, t, *b, m));
    if WEIGHTS.iter().any(|w| w.0 == STICK_TIER && fits(w) != Some(false)) {
        return (STICK_TIER, Source::Stick);
    }
    match WEIGHTS.iter().rev().find(|w| fits(w) == Some(true)) {
        Some(&(t, _)) => (t, Source::Largest),
        None => (WEIGHTS[0].0, Source::Smallest),
    }
}
