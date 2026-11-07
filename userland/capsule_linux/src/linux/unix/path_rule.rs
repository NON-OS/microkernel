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


//! Which path names the display server. Pure, so the host proofs hold it.

/// The display socket a Wayland client looks for.
const DISPLAY: &[u8] = b"wayland-0";

/// The path's last component is the display's, whatever directory it is
/// in: `xwayland-0` or `my-wayland-0` is some other socket.
pub fn is_display(path: &[u8]) -> bool {
    let last = path.rsplit(|&b| b == b'/').next().unwrap_or(path);
    last == DISPLAY
}
