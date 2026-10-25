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

//! Publish (B8, plan/31): sign a listing as its publisher. The person types
//! the path of its entry document under /data; the screen shows the binding
//! and each release's attest, and signs only what the person presses.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::screen::shield::parts::field;
use crate::wallet::state::State;

const LEAD: &str = "Sign a listing as its publisher. Type the path of its entry document \
     under /data; each signature is written beside it for marketplace-index.";

pub fn show(state: &State, fb: &mut PaintBuffer) {
    if let Some(s) = &state.publisher.sending {
        return super::review::review(state, s, fb);
    }
    if let Some(i) = state.publisher.attest_ask {
        return super::attest_screen::show(state, i, fb);
    }
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let footer = [("Open", Weight::Primary, true)];
    let spec = FrameSpec {
        number: "11",
        title: "Publish",
        back: true,
        backdrop: Some(Backdrop::Settings),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    let p = &state.publisher;
    let (at, used) = field(fb, c, y, "ENTRY DOCUMENT", &p.path, "/data/my-app.entry", p.focus == 0);
    hits::put(Press::Field(0), at);
    y += used + GAP;
    if let Some(news) = &p.news {
        y += fact(fb, c.x, y, c.w, "last signed", news);
    }
    if let Some(e) = &p.entry {
        y += super::rows::entry(state, e, fb, c, y);
        y += super::acts::acts(state, e, fb, c, y);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[true]);
}
