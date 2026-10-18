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

//! What the lying-controller proofs script: the keyboard's key report and
//! a link the controller says it opened.

/// The boot keyboard input report's value handle in `keyboard_mouse`.
pub const KEYS: u16 = 11;
/// A key down: the boot report with A (0x04) pressed.
pub const A_DOWN: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];

/// LE Connection Complete (Vol 4, Part E, 7.7.65.1) for `handle`, as
/// central, from the air's peer.
pub fn opened(handle: u16) -> Vec<u8> {
    let [h0, h1] = handle.to_le_bytes();
    let mut e = vec![0x3e, 19, 0x01, 0x00, h0, h1, 0x00, 0x01];
    e.extend_from_slice(&crate::air::PEER);
    e.extend_from_slice(&[0x20, 0x00, 0x00, 0x00, 0x90, 0x01, 0x00]);
    e
}
