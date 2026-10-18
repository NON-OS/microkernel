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

//! What is left: of the pairing window, and of each device's battery.

use nonos_bt_proto::ops::PAIR_MS;
use nonos_bt_proto::Link;

use super::known::Panel;
use super::reach::Reach;

/// The pairing window: shut, or open with the seconds left when this
/// window opened it and so knows when.
pub fn window(p: &Panel, now_ms: i64) -> Option<Option<u64>> {
    if p.reach != Reach::Answered || !p.has(Link::WINDOW) {
        return None;
    }
    Some(p.window_from_ms.map(|from| seconds_left(from, now_ms)))
}

/// Between polls while the page is open: quicker while a device may pair,
/// so what pairing asks shows within a quarter second of being asked.
pub fn poll_ms(p: &Panel) -> i64 {
    if p.has(Link::WINDOW) || p.has(Link::WAITING) {
        250
    } else {
        1000
    }
}

/// The lowest level kept for the bond at `place`: the one that runs out
/// first, when a device has more than one battery.
pub fn level_of(p: &Panel, place: usize) -> Option<u8> {
    let levels = &p.levels[..p.level_count];
    levels.iter().filter(|l| usize::from(l.place) == place).map(|l| l.percent).min()
}

/// Whole seconds left of a window opened at `from`, rounded up, so the page
/// reads 1 until the last moment and never 0 while it is still open.
fn seconds_left(from: i64, now_ms: i64) -> u64 {
    let gone = u64::try_from(now_ms.saturating_sub(from)).unwrap_or(0);
    PAIR_MS.saturating_sub(gone).div_ceil(1000)
}
