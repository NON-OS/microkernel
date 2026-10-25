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

//! When the band is drawn whole. It is drawn clipped to each damaged
//! rectangle, so a change in what it shows, a mark lit or put out, would
//! reach the screen only where some damage fell and leave the rest as it
//! was. Each frame compares what the band shows now with what it was last
//! drawn showing, and a difference damages all of it, whatever made the
//! change: a capturer, a screenshot, the trusted band taken or let go.

const CAMERA: u32 = 1;
const MICROPHONE: u32 = 2;
const SCREENSHOT: u32 = 4;
const TRUSTED: u32 = 8;
/// Set in every state, so no state is the word of a band never drawn (0).
const SHOWN: u32 = 0x100;

/// What the band shows, in one word.
pub fn shows(camera: bool, microphone: bool, screenshot: bool, trusted: bool) -> u32 {
    let bit = |on: bool, b: u32| if on { b } else { 0 };
    SHOWN
        | bit(camera, CAMERA)
        | bit(microphone, MICROPHONE)
        | bit(screenshot, SCREENSHOT)
        | bit(trusted, TRUSTED)
}

/// Whether the band is drawn whole this frame: it was last drawn showing
/// `drawn` (0 never), and shows `now`.
pub fn due(drawn: u32, now: u32) -> bool {
    drawn != now
}
