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

//! 01  NONOS, the first screen: make a wallet or restore one. The words are
//! the phones' shape with this machine's facts: the keys are made here, and
//! the balance is read from a public node, which this wallet says plainly.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::frame;
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::tokens::{GAP, TEXT, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;

const STATEMENT: &str = "The keys are made on this machine and never leave it.";
const LEAD: &str = "Balances are read over TLS from a public Ethereum node, which sees \
    which address asks. Private payments come with Shield.";

pub fn welcome(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let footer = [
        ("Create a wallet", Weight::Primary, true),
        ("Import a private key", Weight::Secondary, true),
    ];
    let spec = FrameSpec {
        number: "01",
        title: "N\u{d8}NOS",
        back: false,
        backdrop: Some(Backdrop::Welcome),
        failure: None,
        footer: &footer,
        status: &status,
    };
    let l = frame(fb, &spec);
    let c = l.content;
    crate::wallet::paint::logo::logo(fb, c.x, c.y, 44);
    let mut y = (c.y + 44 + GAP) as i32;
    y += wrapped(fb, c.x as i32, y, c.w as i32, Role::Statement, STATEMENT, TEXT) + GAP as i32;
    wrapped(fb, c.x as i32, y, c.w as i32, Role::Lead, LEAD, TEXT_3);
    hits::put(Press::Footer(0), l.footer[0]);
    hits::put(Press::Footer(1), l.footer[1]);
}
