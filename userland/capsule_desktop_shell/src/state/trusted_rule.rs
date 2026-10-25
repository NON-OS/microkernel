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

//! When the shell's chrome belongs in the compositor's trusted band, and how
//! it moves there and back. A permission prompt, the system's own prompt and
//! the hidden screen are drawn on the chrome; while one is up the chrome is
//! placed in the band, where no window can cover it, no screenshot holds it,
//! and the router gives the shell every key and click.

use super::attention::Attention;

/// The compositor's trusted band (its `state/trusted_rule.rs`), and the
/// chrome's own band under it (`setup/prime/register.rs`).
pub const TRUSTED_Z: u32 = 0x4000_0000;
pub const CHROME_Z: u32 = 3_000_000;

/// Whether the chrome should be in the trusted band: a prompt or the hidden screen
/// is up. Menus and the Launchpad are the shell's but ask nothing.
pub fn in_band(prompt: bool, attention: Attention) -> bool {
    prompt || attention != Attention::None
}

/// From the band the chrome is in (`up`) to the one it should be in: the
/// band to place it in first and the one to take it from after, so it is
/// never shown in neither. None when it is where it should be.
pub fn moves(up: bool, want: bool) -> Option<(u32, u32)> {
    match (up, want) {
        (false, true) => Some((TRUSTED_Z, CHROME_Z)),
        (true, false) => Some((CHROME_Z, TRUSTED_Z)),
        _ => None,
    }
}

/// Whether the shell answers Ctrl+Alt+T: never while the chrome is in the
/// band (`up`), so no Terminal opens over a prompt or the hidden screen.
pub fn opens_terminal(up: bool) -> bool {
    !up
}
