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

//! The generator's scheduled reseed. After RESEED_INTERVAL blocks it takes a
//! fresh seed from every entropy source and folds in its own next output, so
//! a state read out of memory stops predicting what comes after. When the
//! sources cannot answer, it keeps its state and asks again on the next draw.

use super::super::csprng::ChaChaRng;
use super::super::entropy::collect_seed_entropy_secure;
use crate::crypto::hash::sha256;

pub(super) fn draw(rng: &mut ChaChaRng, buf: &mut [u8]) {
    rng.fill_bytes(buf);
    if !rng.needs_reseed() {
        return;
    }
    let Ok(fresh) = collect_seed_entropy_secure() else {
        return;
    };
    let mut both = [0u8; 64];
    both[..32].copy_from_slice(&fresh);
    rng.fill_bytes(&mut both[32..]);
    rng.reseed(sha256(&both));
    for b in both.iter_mut() {
        // SAFETY: `b` is a live, exclusively borrowed byte of `both`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
}
