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
//! Why the staking button takes no press, said under the form.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;

/// The refusal line; takes the running `y` and returns it.
pub(super) fn why(
    fb: &mut PaintBuffer,
    c: Rect,
    mut y: u32,
    state: &State,
    mainnet: bool,
    working: Option<&str>,
) -> u32 {
    /* A button that takes no press says why. */
    let why = crate::wallet::event::stake_refusal(state)
        .and_then(|w| core::str::from_utf8(w).ok())
        .or_else(|| crate::wallet::send::held_back(state));
    if let (true, None, Some(why)) = (mainnet, working, why) {
        let (x, w) = (c.x as i32, c.w as i32);
        y += wrapped(fb, x, y as i32, w, Role::Lead, why, TEXT_3) as u32 + GAP / 2;
    }
    y
}
