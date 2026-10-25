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

//! A purchase the store named (B8, the plan): the market's terms, Buy or
//! Close, then each step as the meter takes it.

use nonos_app_skeleton::PaintBuffer;

use super::words::{closable, lead as first};
use super::words_voucher::another;
use crate::wallet::arm::gate::greyed;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::order::terms::shown::opens;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::edges;
use crate::wallet::state::State;

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let Some(p) = state.meter.purchase.as_deref() else { return };
    if !opens(p.reading.as_ref(), &p.read, p.record.as_ref()) {
        return;
    }
    let consent = super::consent::lines(p);
    let (sentence, action) = consent.as_ref().map_or_else(|| first(p), |l| (&*l.lead, l.action));
    // A consent's press only once the focus guard armed it (terms/arming.rs),
    // and a budget's or an escrow's once its own guard did (arm).
    let guarded = crate::wallet::order::guard::poll::guarded(p);
    let act = action.is_some() && (!guarded || p.armed) && !greyed(state, Press::Footer(1));
    let close = closable(p);
    let status = crate::wallet::screen::status::parts(state);
    let mut footer = alloc::vec![
        ("Close", Weight::Secondary, close),
        (action.unwrap_or(""), Weight::Primary, act)
    ];
    footer.extend(another(p).map(|a| (a, Weight::Secondary, true)));
    let spec = FrameSpec {
        number: "13",
        title: "Purchase",
        back: false,
        backdrop: Some(Backdrop::Send),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let y = super::body::paint(p, consent.as_ref(), fb, c, sentence);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &footer.iter().map(|f| f.2).collect::<alloc::vec::Vec<_>>());
}
