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

//! The first frame waits for something to show. Until the compositor
//! presents, the screen still holds what the firmware left on it (its logo,
//! on most machines), and an empty desktop painted over that is the flash
//! between firmware and splash. So nothing goes to the display until a
//! client has placed a layer (the splash paints before it submits) or
//! FIRST_FRAME_MS has passed, so a boot with no splash is not left dark.
//! Damage is kept meanwhile, so the first frame is the whole screen.

/// Long enough for the splash to find the compositor, short enough that a
/// splash which never comes costs little.
pub const FIRST_FRAME_MS: i64 = 3000;

/// Whether the first frame is still held: `since` is when the hold began
/// (None once released), `layers` how many layers the scene holds.
pub fn held(since: Option<i64>, now_ms: i64, layers: usize) -> bool {
    since.is_some_and(|t| layers == 0 && now_ms.saturating_sub(t) < FIRST_FRAME_MS)
}
