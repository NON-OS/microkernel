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

use alloc::vec::Vec;

use super::track_size::Track;

/* Share `free` px out among the tracks: first growing content tracks up to
 * their limits in equal steps, then to the flexible tracks by fraction,
 * or, with none of those, stretching the auto tracks when `stretch` holds.
 * Returns what is still left over. */
pub(in super::super) fn grow(tracks: &mut [Track], mut free: i32, stretch: bool) -> i32 {
    while free > 0 {
        let open: Vec<usize> = (0..tracks.len())
            .filter(|&i| tracks[i].fr == 0 && tracks[i].base < tracks[i].limit)
            .collect();
        if open.is_empty() {
            break;
        }
        let share = (free / open.len() as i32).max(1);
        for i in open {
            let add = share.min(tracks[i].limit - tracks[i].base).min(free);
            (tracks[i].base, free) = (tracks[i].base + add, free - add);
        }
    }
    let frs: i64 = tracks.iter().map(|t| t.fr).sum();
    if frs > 0 && free > 0 {
        let unit = frs.max(100);
        tracks
            .iter_mut()
            .filter(|t| t.fr > 0)
            .for_each(|t| t.base = t.base.max((free as i64 * t.fr / unit) as i32));
        return 0;
    }
    let autos = tracks.iter().filter(|t| t.auto).count() as i32;
    if stretch && autos > 0 && free > 0 {
        let (each, mut rest) = (free / autos, free % autos);
        for t in tracks.iter_mut().filter(|t| t.auto) {
            t.base += each + (rest > 0) as i32;
            rest -= 1;
        }
        return 0;
    }
    free
}
