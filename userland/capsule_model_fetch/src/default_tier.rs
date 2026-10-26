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
 * 3. Else the largest proven tier that fits this machine's memory.
 * 4. Else the smallest proven tier, which then says it does not fit.
 *
 * Only a proven tier (`PROVEN`) is ever a default: one whose model's
 * architecture has answered on a NONOS boot. Any other tier can still be
 * chosen, installed and run; it is never started, or downloaded, for a
 * person who did not ask for it.
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
    ("qwen3.5-0.8b", 833_592_096),
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
    ("qwen3.6-27b", 19_095_766_304),
    ("qwen3-32b", 19_762_149_024),
    ("coder-32b", 19_851_336_128),
    ("max", 19_851_336_384),
];

/*
 * The tiers proven on NONOS, each with the boot that proved it: its
 * architecture (as llama.cpp names it) answered a question on that boot.
 * A tier's mark is added in its own commit, naming its evidence, when a
 * tier of its architecture boots to an answer. Not here: qwen3-30b-a3b
 * (qwen3moe), whose smallest published size has not run on NONOS, and
 * qwen3.5-0.8b and qwen3.6-27b (qwen35) until a tier of theirs answers.
 */
#[rustfmt::skip]
pub const PROVEN: &[(&str, &str)] = &[
    ("small", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("medium", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("large", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("xlarge", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("xxl", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("max", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-1.5b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-7b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-14b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-32b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("qwen3-0.6b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-1.7b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-4b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-8b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-14b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-32b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
];

/* Whether `tier` is proven on NONOS, so may be a default. */
pub fn proven(tier: &str) -> bool {
    PROVEN.iter().any(|(t, _)| *t == tier)
}

const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn stick_proven() -> bool {
    let mut i = 0;
    while i < PROVEN.len() {
        if same(PROVEN[i].0, STICK_TIER) {
            return true;
        }
        i += 1;
    }
    false
}

/* The stick tier is a default before any other, so it must be proven. */
const _: () = assert!(stick_proven(), "the stick tier is not proven on NONOS");

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
    resolve_in(WEIGHTS, proven, chosen, memory, room)
}

/* `resolve` over a table of weights and a proof mark, so a proof can hold
 * the rule to tables the shipped ones do not reach. */
pub fn resolve_in(
    weights: &'static [(&'static str, u64)],
    proven: impl Fn(&str) -> bool,
    chosen: &[u8],
    memory: Option<u64>,
    room: Room,
) -> (&'static str, Source) {
    let word_byte = |c: &u8| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'.' || *c == b'-';
    let word = &chosen[..chosen.iter().position(|c| !word_byte(c)).unwrap_or(chosen.len())];
    if let Some(&(t, _)) = weights.iter().find(|(t, _)| t.as_bytes() == word && !word.is_empty()) {
        return (t, Source::Chosen);
    }
    let fits = |(t, b): &(&str, u64)| memory.and_then(|m| tier_fits(room, t, *b, m));
    if weights.iter().any(|w| w.0 == STICK_TIER && proven(w.0) && fits(w) != Some(false)) {
        return (STICK_TIER, Source::Stick);
    }
    match weights.iter().rev().find(|w| proven(w.0) && fits(w) == Some(true)) {
        Some(&(t, _)) => (t, Source::Largest),
        None => (weights.iter().find(|w| proven(w.0)).map_or(STICK_TIER, |w| w.0), Source::Smallest),
    }
}
