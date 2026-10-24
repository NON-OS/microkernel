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

//! When the bound port's connect status is read: about once a poll, while
//! the device plays as much as while it is idle. Whether a stream to a
//! device pulled out mid-playback fails its batches is up to how driver.xhci0
//! reports a port that is gone; the connect status is not, so a device taken
//! out while it plays is noticed within a poll either way. Pure, for the
//! proofs.

/// Read the port now: a device is bound and a poll is due. Playing or not
/// does not matter.
pub fn look_now(bound: bool, _playing: bool, due: bool) -> bool {
    bound && due
}
