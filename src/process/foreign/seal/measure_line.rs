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

//! The owner's measuring line for any program a family ran (plan/32):
//! `[MEASURE] app=`, the first 8 bytes of its app id in hex, ` ok=`, then
//! the 130 hex digits of nonos.prove's `[PROVE] syscall buckets ok=` line,
//! bit b set when a call in bucket b went through. A program the vendor
//! ships has no store listing to prove, so this is how its calls= set is
//! measured. Only a build with capsule serial output says it (finish.rs),
//! and no release profile keeps that.

/// Bytes of the set: 513 buckets, one bit each.
const SET: usize = 65;
const HEAD: &[u8] = b"[MEASURE] app=";
const OK: &[u8] = b" ok=";
pub(super) const LEN: usize = HEAD.len() + 16 + OK.len() + 2 * SET;

const HEX: &[u8; 16] = b"0123456789abcdef";

/// The line for app `id` whose family's histogram is `hist`: two words a
/// bucket, the first counting the calls that went through.
pub(super) fn line(id: &[u8; 32], hist: &[u32]) -> [u8; LEN] {
    let mut set = [0u8; SET];
    for (b, pair) in hist.chunks_exact(2).enumerate().take(SET * 8) {
        if pair[0] > 0 {
            set[b / 8] |= 1 << (b % 8);
        }
    }
    let mut out = [0u8; LEN];
    out[..HEAD.len()].copy_from_slice(HEAD);
    let mut at = HEAD.len();
    id[..8].iter().for_each(|&b| hex(&mut out, &mut at, b));
    out[at..at + OK.len()].copy_from_slice(OK);
    at += OK.len();
    set.iter().for_each(|&b| hex(&mut out, &mut at, b));
    out
}

fn hex(out: &mut [u8], at: &mut usize, byte: u8) {
    out[*at] = HEX[usize::from(byte >> 4)];
    out[*at + 1] = HEX[usize::from(byte & 15)];
    *at += 2;
}
