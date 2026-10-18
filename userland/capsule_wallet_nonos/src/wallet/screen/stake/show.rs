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
//! The staking screen: the review when one is open, else the form.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits::{self, Press};
use super::super::page::{edges, lead};
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::shield::parts::chips;
use crate::wallet::state::State;

const LEAD: &str = "Staked NOX earns a share of the emission. A lock term weighs the stake \
     more and keeps it in until the term ends.";
const NO_STAKING: &str = "Staking runs on Ethereum mainnet. Switch networks in Settings to \
     stake.";

pub fn show(state: &State, fb: &mut PaintBuffer) {
    if let Some(d) = &state.stake.draft {
        return super::review::review(state, fb, d);
    }
    hits::clear();
    let status = super::super::status::parts(state);
    let mainnet = chain::current().staking.is_some();
    /* While a transaction goes out the button says so and takes no press. */
    let working = crate::wallet::act::working(state);
    let label = working.map_or_else(|| super::text::action(state), String::from);
    let ok = mainnet
        && working.is_none()
        && crate::wallet::event::stake_refusal(state).is_none()
        && crate::wallet::send::held_back(state).is_none();
    let footer =
        [(label.as_str(), Weight::Primary, ok), ("Use all NOX", Weight::Secondary, mainnet)];
    let shown = if state.stake.unstake == 1 { &footer[..1] } else { &footer[..] };
    let spec = FrameSpec {
        number: "07",
        title: "Stake NOX",
        back: true,
        backdrop: Some(Backdrop::Deposit),
        failure: state.failure,
        footer: shown,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, if mainnet { LEAD } else { NO_STAKING });
    y += chips(fb, c, y, &["Stake", "Unstake"], Some(state.stake.unstake), Press::Tab) + GAP;
    y = super::form::form(fb, c, y, state);
    y = super::why::why(fb, c, y, state, mainnet, working);
    y = super::facts::facts(fb, c, y, state);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, mainnet && state.stake.unstake == 0]);
}
