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

/* The three averages, decayed as Linux decays them. */

use super::super::declared::{CPUS, HZ};
use super::state::LOAD;

const FIXED_1: u64 = 1 << 11;

/* Linux's EXP_1, EXP_5 and EXP_15: 2048/exp(5s/1min), /exp(5s/5min), /exp(5s/15min). */
const EXP: [u64; 3] = [1884, 2014, 2037];

const PERIOD_MS: u64 = 5000;

/* After this many periods every average has reached the share it decays to. */
const SETTLED: u64 = 4096;

/*
 * The three averages in Linux's fixed point, at `now_ms` since the family
 * started, with the family's live threads having run `live` ticks.
 */
pub fn averages(now_ms: u64, live: u64) -> [u64; 3] {
    let mut s = LOAD.0.borrow_mut();
    let ran = live + s.gone;
    let periods = now_ms.saturating_sub(s.at_ms) / PERIOD_MS;
    if periods > 0 {
        let span = periods * PERIOD_MS * HZ / 1000;
        let share = (ran.saturating_sub(s.ran) * FIXED_1 / span).min(FIXED_1 * CPUS);
        for (avg, exp) in s.avg.iter_mut().zip(EXP) {
            for _ in 0..periods.min(SETTLED) {
                *avg = decay(*avg, exp, share);
            }
        }
        s.at_ms += periods * PERIOD_MS;
        s.ran = ran;
    }
    s.avg
}

/*
 * Linux's calc_load: one period's decay toward `share`, rounded up while
 * rising.
 */
fn decay(avg: u64, exp: u64, share: u64) -> u64 {
    let next = avg * exp + share * (FIXED_1 - exp);
    (next + if share >= avg { FIXED_1 - 1 } else { 0 }) / FIXED_1
}

/* "0.42", as /proc/loadavg writes an average, rounded as Linux rounds it. */
pub fn text(avg: u64) -> alloc::string::String {
    let v = avg + FIXED_1 / 200;
    alloc::format!("{}.{:02}", v >> 11, ((v & (FIXED_1 - 1)) * 100) >> 11)
}
