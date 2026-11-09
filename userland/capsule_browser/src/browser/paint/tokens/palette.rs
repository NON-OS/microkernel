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

use core::sync::atomic::{AtomicU8, Ordering};

use nonos_toolkit::theme::{roles, snapshot};

use super::{system, DARK, LIGHT};
use crate::browser::shell::kept::Appearance;

/// The colours of the browser's own pages under one appearance; the chrome
/// around them takes the OS theme's roles, as the shared widgets do. Text
/// tokens (`ink`, `ink2`, `muted`) read on every ground a tile is lifted
/// against (`ground`, `surface`, `sidebar`, `field`). Every token is opaque
/// but `shadow`, so a painter may fill with any of the others.
#[derive(Clone, Copy)]
pub struct Palette {
    pub ground: u32,
    pub surface: u32,
    pub sidebar: u32,
    pub field: u32,
    pub ink: u32,
    pub ink2: u32,
    pub muted: u32,
    pub accent: u32,
    pub accent_soft: u32,
    pub sky: u32,
    pub shadow: u32,
}

/* One appearance for the whole process: every window and tab paints under
 * it. It starts on System, the default, so a frame drawn before the saved
 * choice is read already follows the OS theme. */
static MODE: AtomicU8 = AtomicU8::new(Appearance::System as u8);

/// The palette the pages paint with now. System is worked out from the OS
/// theme's roles on every call, so a change made in Settings shows on the
/// next paint.
pub fn palette() -> Palette {
    match mode() {
        Appearance::Light => LIGHT,
        Appearance::Dark => DARK,
        Appearance::System => system::from_roles(&roles()),
    }
}

/// Puts the appearance `a` in force.
pub fn set_mode(a: Appearance) {
    MODE.store(a as u8, Ordering::Relaxed);
}

/// The appearance in force.
pub fn mode() -> Appearance {
    Appearance::from_byte(MODE.load(Ordering::Relaxed))
}

/// What the colours in force are worked out from: the appearance, and the OS
/// theme's revision, which moves only when Settings changes the theme. The
/// chrome follows the OS theme under every appearance, so the revision counts
/// under all three. While the key holds still, so does every colour drawn.
pub fn key() -> u64 {
    (mode() as u64) << 32 | snapshot().revision as u64
}
