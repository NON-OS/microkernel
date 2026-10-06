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

use super::band_choice::{choose, BANDS};

/// The scan the choice replaced: band by band, the lowest pid above the
/// band's last pick, else the band's lowest pid.
fn band_by_band(candidates: &[(u32, usize)], last: &[u32; BANDS]) -> Option<(u32, usize)> {
    for (band, &band_last) in last.iter().enumerate() {
        let pids = candidates.iter().filter(|c| c.1 == band).map(|c| c.0);
        let after = pids.clone().filter(|&p| p > band_last).min();
        if let Some(pid) = after.or(pids.min()) {
            return Some((pid, band));
        }
    }
    None
}

/// A small linear congruential generator, so the cases are the same on
/// every run.
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *seed >> 33
}

#[test]
fn the_choice_matches_the_band_by_band_scan_on_every_generated_case() {
    let mut seed = 7;
    for _ in 0..20_000 {
        let n = (next(&mut seed) % 12) as usize;
        let candidates: Vec<(u32, usize)> = (0..n)
            .map(|_| ((next(&mut seed) % 40) as u32 + 1, (next(&mut seed) % BANDS as u64) as usize))
            .collect();
        let last = core::array::from_fn(|_| (next(&mut seed) % 42) as u32);
        assert_eq!(choose(&candidates, &last), band_by_band(&candidates, &last));
    }
}

#[test]
fn the_highest_band_wins_and_turns_go_round_by_pid() {
    let c = [(9, 2), (4, 2), (30, 1), (12, 1)];
    assert_eq!(choose(&c, &[0; BANDS]), Some((12, 1)));
    assert_eq!(choose(&c, &[0, 12, 0, 0, 0]), Some((30, 1)));
    assert_eq!(choose(&c, &[0, 30, 0, 0, 0]), Some((12, 1)));
    assert_eq!(choose(&[(9, 2), (4, 2)], &[0, 0, 4, 0, 0]), Some((9, 2)));
}

#[test]
fn nothing_to_choose_is_none() {
    assert_eq!(choose(&[], &[0; BANDS]), None);
    assert_eq!(choose(&[(3, BANDS)], &[0; BANDS]), None);
}
