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

//! A window's mode and the configure that asks for it. Pure, so the host
//! proofs hold it.

/// x, y, width, height on screen.
pub type Rect = (u32, u32, u32, u32);

/// xdg_toplevel.state: maximized is 1, fullscreen 2.
pub const STATE_MAXIMIZED: u32 = 1;
pub const STATE_FULLSCREEN: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Maximized,
    FullScreen,
}

impl Mode {
    /// The states a configure for this mode carries.
    pub fn states(self) -> &'static [u32] {
        match self {
            Mode::Normal => &[],
            Mode::Maximized => &[STATE_MAXIMIZED],
            Mode::FullScreen => &[STATE_FULLSCREEN],
        }
    }

    /// Whether the window fills the display, which the window manager is
    /// told with its full-screen flag.
    pub fn fills(self) -> bool {
        self != Mode::Normal
    }
}

/// One configure sent to the guest. `rect` None asks the guest to pick its
/// own size (a configure of 0 by 0), for a window that filled the display
/// before it was ever shown smaller.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Configure {
    pub serial: u32,
    pub mode: Mode,
    pub rect: Option<Rect>,
}

impl Configure {
    /// The size the configure names, 0 by 0 for the guest's own choice.
    pub fn size(&self) -> (u32, u32) {
        self.rect.map_or((0, 0), |r| (r.2, r.3))
    }
}
