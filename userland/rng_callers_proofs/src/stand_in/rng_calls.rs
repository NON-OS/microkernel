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

// The refusing calls the callers must use, included into `rng`, and the
// choice of which draws refuse.

/// Draws from the `n`th on (from 1) refuse; `None`: all answer. Clears all.
pub fn fail_from(n: Option<u64>) {
    fail_at(n, false);
}

/// Each way a call of `k` draws can meet a refusal: from each draw on, and
/// at that draw alone.
pub fn cases(k: u64) -> impl Iterator<Item = (u64, bool)> {
    (1..=k).flat_map(|n| [(n, false), (n, true)])
}

/// Only the `n`th draw refuses, and every later one answers.
pub fn fail_only(n: u64) {
    fail_at(Some(n), true);
}

/// Draws from the `n`th on refuse, or with `only`, just the `n`th.
pub fn fail_at(n: Option<u64>, only: bool) {
    DRAWS.set(0);
    FAIL_FROM.set(n);
    ONLY.set(only);
    WEAK.set(0);
    GIVEN.with_borrow_mut(Vec::clear);
}

/// The seeded draw: refuses, leaving `buf` as it was, from the set draw on.
pub fn fill_random_bytes_secure(buf: &mut [u8]) -> RngResult<()> {
    let n = DRAWS.get() + 1;
    DRAWS.set(n);
    if FAIL_FROM.get().is_some_and(|at| n == at || (n > at && !ONLY.get())) {
        return Err(RngError::NotInitialized);
    }
    for chunk in buf.chunks_mut(8) {
        chunk.copy_from_slice(&next_word().to_le_bytes()[..chunk.len()]);
    }
    GIVEN.with_borrow_mut(|g| g.push(buf.to_vec()));
    Ok(())
}

pub fn random_u64_secure() -> RngResult<u64> {
    let mut b = [0u8; 8];
    fill_random_bytes_secure(&mut b)?;
    Ok(u64::from_le_bytes(b))
}
