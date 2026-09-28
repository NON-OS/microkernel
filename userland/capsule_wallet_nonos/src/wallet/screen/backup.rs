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

//! 02  RECOVERY PHRASE, shown once, straight after the wallet is made. The
//! words are the wallet: without them a machine that cannot keep keys loses
//! the account at the next reboot. There is no back button; the only way on
//! is to say they are written down, which wipes them from this machine.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::tokens::{GAP, TEXT_2, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;

const LEAD: &str = "Write these words on paper, in this order. They are shown once.";
const WARN: &str = "Whoever holds them holds the account. Never type them into a website \
                    and never photograph them.";

pub fn backup(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let footer = [("I wrote them down", Weight::Primary, true)];
    let spec = FrameSpec {
        number: "02",
        title: "Recovery phrase",
        back: false,
        backdrop: Some(Backdrop::Backup),
        failure: None,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let (x, w) = (c.x as i32, c.w as i32);
    let mut y = c.y + wrapped(fb, x, c.y as i32, w, Role::Lead, LEAD, TEXT_2) as u32 + GAP;
    y += super::backup_words::grid(state, fb, c.x, y, c.w) + GAP;
    y += wrapped(fb, x, y as i32, w, Role::Fact, WARN, TEXT_3) as u32 + GAP;
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    hits::put(Press::Footer(0), l.footer[0]);
}
