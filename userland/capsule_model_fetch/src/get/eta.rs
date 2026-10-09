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
 * The time left of a download, said only when it can be said honestly:
 * from the rate bytes came at in this run, once at least `MEASURED_MS` and
 * `MEASURED_BYTES` have been measured, so a first burst or a slow start is
 * never taken for the rate. Pure, so model_fetch_proofs holds it.
 */

/* The time left is said only once this long and this much have been measured. */
pub const MEASURED_MS: u64 = 10_000;
pub const MEASURED_BYTES: u64 = 4 << 20;

/*
 * Seconds left for `left` bytes, from `got` bytes measured over `ms`: None
 * until enough has been measured for the rate to mean anything.
 */
pub fn eta(left: u64, got: u64, ms: u64) -> Option<u64> {
    if ms < MEASURED_MS || got < MEASURED_BYTES {
        return None;
    }
    let rate = got.saturating_mul(1_000) / ms;
    (rate > 0).then(|| left / rate)
}
