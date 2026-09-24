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

//! The frame: ground, header, the screen's body in the middle band, footer.
//! Order is load-bearing; the window is opaque and each layer paints over
//! the one before.

use nonos_app_skeleton::PaintBuffer;

use super::metrics::{FOOTER_H, HEADER_H, PAD};
use super::{footer, header, screens, theme};
use crate::install::state::{Screen, State};

/// The rectangle a screen owns, below the header and above the footer.
#[derive(Clone, Copy)]
pub struct Body {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(theme::BACKGROUND);
    let (w, h) = (fb.width, fb.height);
    header::paint(fb, state.screen, w);
    let body = Body {
        x: PAD,
        y: HEADER_H + PAD / 2,
        w: w.saturating_sub(2 * PAD),
        h: h.saturating_sub(HEADER_H + FOOTER_H + PAD),
    };
    match state.screen {
        Screen::Welcome => screens::welcome::paint(state, fb, body),
        Screen::Disks => screens::disks::paint(state, fb, body),
        Screen::Confirm => screens::confirm::paint(state, fb, body),
        Screen::Writing | Screen::Verifying => screens::writing::paint(state, fb, body),
        Screen::Done => screens::done::paint(state, fb, body),
        Screen::Failed => screens::failed::paint(state, fb, body),
    }
    footer::paint(fb, state, w, h);
}
