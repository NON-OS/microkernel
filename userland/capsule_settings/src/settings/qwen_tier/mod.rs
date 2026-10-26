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

//! The "Qwen model" row: a choice among the pinned tiers, not free text.
//!
//! The tiers and their names are setup's own table (`labels.rs`), which
//! setup's build holds to the Linux personality's pins, so Settings offers
//! the tiers that exist, all but a vision tier, which needs an image. Left
//! and Right step through them and wrap;
//! Enter steps on. Nothing set reads as the default, which qwen and the
//! dock pick by one rule (capsule_model_fetch/src/default_tier.rs) and say. The value goes to the policy store as the tier word, the
//! one the Terminal reads on every `qwen` command and the policy service
//! keeps on a machine that keeps state. Pure, so capsule_settings_proofs
//! holds it.

#[path = "../../../../capsule_setup_wizard/src/qwen/labels.rs"]
mod labels;

use labels::{label, LABELS};

/*
 * The tiers the row offers: every pinned tier but a vision one (need.rs's
 * VISION). A chosen tier is what a plain `qwen` runs with no image, so it is
 * a text model; vl-2b is opened by name with its picture,
 * `qwen vl-2b QUESTION < FILE`.
 */
const VISION: &[&[u8]] = &[b"vl-2b"];

fn offered() -> impl Iterator<Item = &'static [u8]> {
    LABELS.iter().map(|(t, _)| *t).filter(|t| !VISION.contains(t))
}

/// The tier `delta` rows on from `current`, wrapping at either end. From
/// nothing chosen, or a tier the row does not offer, Right takes the first
/// tier and Left the last.
pub fn step(current: &[u8], delta: i32) -> &'static [u8] {
    let n = offered().count() as i64;
    let at = match offered().position(|t| t == current) {
        Some(row) => (row as i64 + i64::from(delta)).rem_euclid(n),
        None if delta >= 0 => 0,
        None => n - 1,
    };
    offered().nth(at as usize).unwrap_or(b"")
}

/// What the row shows for `current`: the tier chosen, or that none is and
/// the default runs (the stick tier when it fits, else the largest that
/// fits, as qwen and the dock pick it and say it).
pub fn shown(current: &[u8]) -> &'static [u8] {
    match LABELS.iter().find(|(t, _)| *t == current) {
        Some(&(t, _)) => label(t),
        None => DEFAULT,
    }
}

/// Nothing chosen.
pub const DEFAULT: &[u8] = b"Default (none chosen)";
