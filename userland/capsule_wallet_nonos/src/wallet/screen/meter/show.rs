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

//! The meter (B8): how it stands, its address, the budget it pays under and
//! each channel, as its ACCOUNT reply says. Read when the screen opens and
//! on Refresh; the meter keeps the numbers, the wallet only shows them.

use nonos_app_skeleton::PaintBuffer;
use nonos_payment_proto::READY;

use super::budget;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::meter::rows::{budget_rows, state_line};
use crate::wallet::nox::nox_text as nox;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::state::State;

const LEAD: &str = "The meter pays for apps by voucher, from its own address and within the \
     budget set here. Fund it from the Shield, so nothing ties it to this wallet.";

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let ready = budget::ready(state) && !crate::wallet::arm::gate::greyed(state, Press::Footer(1));
    let footer = [("Refresh", Weight::Secondary, true), ("Set budget", Weight::Primary, ready)];
    let spec = FrameSpec {
        number: "10",
        title: "Meter",
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
    let a = state.meter.account.as_ref();
    let line = if state.meter.asked { state_line(a) } else { "not asked yet" };
    y += fact(fb, c.x, y, c.w, "meter", line);
    if let Some(a) = a.filter(|a| a.flags & READY != 0) {
        let address = crate::wallet::screen::receive_address::checksummed_text(&a.address);
        y += fact(fb, c.x, y, c.w, "address", &crate::wallet::etna::groups::shortened(&address));
        for (label, value) in budget_rows(a, crate::wallet::clock::now_s(), nox) {
            y += fact(fb, c.x, y, c.w, label, &value);
        }
        if let Some(news) = &state.meter.news {
            y += fact(fb, c.x, y, c.w, "last job", news);
        }
        y += super::channels::channels(state, a, fb, c, y);
        y += budget::form(state, fb, c, y);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[true, ready]);
}
