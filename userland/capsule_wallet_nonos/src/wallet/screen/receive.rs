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

//! 03  RECEIVE, the public 0x address, as AccountReceiveView.swift shows it:
//! one sentence saying anyone can see it, the address as a QR code and as a
//! grouped value, what is known about it, and a way to copy it.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::qr::qr;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;

const LEAD: &str = "Public: anyone can see what this address holds and sends.";
const QR_SIDE: u32 = 200;

pub fn receive(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let ready = state.address_ready;
    let footer = [("Copy address", Weight::Primary, ready)];
    let spec = FrameSpec {
        number: "03",
        title: "Receive",
        back: true,
        backdrop: Some(Backdrop::Receive),
        failure: None,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, LEAD, TEXT_3) as u32
        + GAP;
    let hex = super::receive_address::address_hex(state);
    if ready {
        let uri = alloc::format!("ethereum:{hex}");
        if qr(fb, c.x, y, QR_SIDE, uri.as_bytes()) {
            y += QR_SIDE + GAP;
        }
        y += value_block(fb, c.x, y, c.w, &hex) + GAP;
    }
    y += super::receive_address::facts(state, fb, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    if let Some(back) = l.back {
        hits::put(Press::Back, back);
    }
    if ready {
        hits::put(Press::Footer(0), l.footer[0]);
    }
}
