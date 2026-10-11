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

use super::hits;
use super::page::{edges, lead};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::meter::channel::channel_line;
use crate::wallet::meter::rows::{budget_rows, state_line};
use crate::wallet::nox::nox_text as nox;
use crate::wallet::state::State;

pub mod click;

const LEAD: &str = "The meter pays for apps by voucher, from its own address and within the \
     budget set here. Fund it from the Shield, so nothing ties it to this wallet.";

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let footer = [("Refresh", Weight::Secondary, true)];
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
        let address = super::receive_address::checksummed_text(&a.address);
        y += fact(fb, c.x, y, c.w, "address", &crate::wallet::etna::groups::shortened(&address));
        for (label, value) in budget_rows(a, nonos_tls::rtc_now(), nox) {
            y += fact(fb, c.x, y, c.w, label, &value);
        }
        for row in &a.rows {
            y += fact(fb, c.x, y, c.w, "channel", &channel_line(row, nox));
        }
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[true]);
}
