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

//! The choice among the candidates a pick collected. Pure, so the host
//! proofs can hold it to the band-by-band scan it replaced.

/// RealTime, High, Normal, Low, Idle: band 0 is taken first.
pub(crate) const BANDS: usize = 5;

/// `candidates` holds each pid this CPU may take, with its band. The choice
/// is the highest band that has one, and within it the lowest pid above
/// that band's last pick, or its lowest pid once every pid has had a turn.
pub(crate) fn choose(candidates: &[(u32, usize)], last: &[u32; BANDS]) -> Option<(u32, usize)> {
    for (band, &band_last) in last.iter().enumerate() {
        let mut after: Option<u32> = None;
        let mut lowest: Option<u32> = None;
        for &(pid, _) in candidates.iter().filter(|&&(_, b)| b == band) {
            lowest = Some(lowest.map_or(pid, |m| m.min(pid)));
            if pid > band_last {
                after = Some(after.map_or(pid, |m| m.min(pid)));
            }
        }
        if let Some(pid) = after.or(lowest) {
            return Some((pid, band));
        }
    }
    None
}
