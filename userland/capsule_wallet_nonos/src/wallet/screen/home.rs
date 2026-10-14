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

//! The home screen, as Overview.swift lays it out: the mark and the way to
//! settings, the account and network, the coins the account holds, and the
//! things a holder does. In a wide window it is two columns.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use super::home_columns::columns;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{BACK, GAP, TEXT_2};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;

pub fn home(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let spec = FrameSpec {
        number: "03",
        title: "Wallet",
        back: false,
        backdrop: Some(Backdrop::Home),
        failure: None,
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    crate::wallet::paint::logo::logo(fb, c.x, c.y + (BACK - 18) / 2, 18);
    let gear = Rect::new(c.x + c.w - BACK, c.y, BACK, BACK);
    crate::wallet::etna::symbol::gear(fb, gear);
    hits::put(Press::Settings, gear);
    let mut top = c.y + BACK + GAP;
    /* A wallet this machine does not keep says so first, in full width. */
    if let Some(note) = state.kept_note {
        let text = super::status_words::capital(note);
        top +=
            wrapped(fb, c.x as i32, top as i32, c.w as i32, Role::Lead, &text, TEXT_2) as u32 + GAP;
    }
    let y = columns(state, fb, c, top);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
}
