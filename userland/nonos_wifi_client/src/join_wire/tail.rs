/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The end of a link reply: how the link last ended and how the last join
//! came out, `[ended u8][joins u8][code i32 LE]` (the drivers'
//! `LinkHistory`), then the association's Transition Disable bitmap. A reply
//! without it, from a driver that predates it, reads as lost and as no join
//! (never as left) and as no Transition Disable.

/// The history's length, then the whole tail's with the bitmap.
const HISTORY_LEN: usize = 6;
pub const TAIL_MAX: usize = HISTORY_LEN + 1;

/// Why the link last ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LinkEnded {
    /// It has not ended since it was last up.
    No,
    /// The access point left, went unheard or let its SA Query go unanswered;
    /// also any reply that does not say.
    #[default]
    Lost,
    /// The person asked to leave.
    Left,
}

/// The last join a driver finished: its count this boot (1 to 255, wrapping
/// past 0), so a reader can tell a new one, and its code, 0 for joined.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LastJoin {
    pub seq: u8,
    pub code: i32,
}

/// The tail at `at` in `body`, or the old reply's reading when it is cut.
pub(super) fn parse_tail(body: &[u8], at: usize) -> (LinkEnded, Option<LastJoin>, u8) {
    let Some(t) = at.checked_add(HISTORY_LEN).and_then(|end| body.get(at..end)) else {
        return (LinkEnded::Lost, None, 0);
    };
    let ended = match t[0] {
        0 => LinkEnded::No,
        2 => LinkEnded::Left,
        _ => LinkEnded::Lost,
    };
    let code = i32::from_le_bytes([t[2], t[3], t[4], t[5]]);
    let td = body.get(at + HISTORY_LEN).copied().unwrap_or(0);
    (ended, (t[1] != 0).then_some(LastJoin { seq: t[1], code }), td)
}
