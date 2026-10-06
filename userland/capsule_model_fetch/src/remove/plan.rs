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
 * Which files taking a tier away removes: every file pinned under it, less
 * any file another tier pins too while that tier is whole on the volume, so
 * removing one tier never breaks another. Today no file is pinned twice
 * (model_fetch_proofs holds that); the rule is here for the day one is.
 * Pure, so model_fetch_proofs holds it.
 */

use alloc::vec::Vec;

/* `pins` is (tier, file name) for every pin; `whole` says a tier is installed. */
pub fn plan<'a>(
    tier: &str,
    pins: &[(&str, &'a [u8])],
    whole: impl Fn(&str) -> bool,
) -> Vec<&'a [u8]> {
    let shared =
        |name: &[u8]| pins.iter().any(|(other, n)| *other != tier && *n == name && whole(other));
    let mut out: Vec<&'a [u8]> = Vec::new();
    for (_, name) in pins.iter().filter(|(t, _)| *t == tier) {
        if !shared(name) && !out.contains(name) {
            out.push(name);
        }
    }
    out
}
