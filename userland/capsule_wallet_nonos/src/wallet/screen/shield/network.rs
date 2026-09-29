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

/*
 * 11  SEPOLIA: the one network Shield runs on for now, and the contracts
 * this wallet talks to there, written out in full so they can be checked
 * against the deployment record rather than trusted.
 */

use nonos_app_skeleton::PaintBuffer;

use super::consts::{CHAIN_ID, FAUCET, POOL, POOL_FROM_BLOCK, RELAYER, VERIFIER};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

const LEAD: &str = "Shield runs on the Sepolia test network for now. Test ETH and NOX \
     come from the faucet, once a day, for this wallet's address.";

pub fn network(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let footer = [("Copy faucet address", Weight::Secondary, true)];
    let spec = FrameSpec {
        number: "11",
        title: "Sepolia",
        back: true,
        backdrop: Some(Backdrop::Settings),
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
    for (name, value) in [("chain id", CHAIN_ID), ("events from block", POOL_FROM_BLOCK)] {
        y += fact(fb, c.x, y, c.w, name, value);
    }
    for (name, value) in
        [("pool", POOL), ("verifier", VERIFIER), ("faucet", FAUCET), ("relayer", RELAYER)]
    {
        y += GAP / 2 + super::pick::label(fb, c, y + GAP / 2, &name.to_uppercase());
        y += value_block(fb, c.x, y, c.w, value);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
    super::edges::footer(&l, &[true]);
}
