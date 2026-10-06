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
//! The wallet with its figures put away: nothing about the account is
//! drawn until the holder asks for it back.

use nonos_app_skeleton::PaintBuffer;

use super::hits;
use super::page::{edges, lead};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::state::State;

const LEAD: &str = "Locked. Balances, the address and every action are hidden, and nothing \
     secret is held on screen, until it is opened again. Opening asks the keyring for this \
     account; the keyring has no passphrase of its own yet.";

pub fn locked(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = ["Locked"];
    let footer = [("Open the wallet", Weight::Primary, true)];
    let spec = FrameSpec {
        number: "",
        title: "N\u{d8}NOS Wallet",
        back: false,
        backdrop: Some(Backdrop::Welcome),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: 0,
    };
    let mut l = begin(fb, &spec);
    let y = lead(fb, l.content, LEAD);
    hits::reach(y, 0, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[true]);
}
