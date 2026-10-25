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

//! The Sepolia page's contract addresses, each under its label.

use nonos_app_skeleton::PaintBuffer;

use super::consts::{FAUCET, POLICY, POOL, REGISTRY, VERIFIER};
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;

/* Draws the contracts from `y` down and returns the y below them. */
pub(super) fn contracts(fb: &mut PaintBuffer, c: Rect, mut y: u32) -> u32 {
    /* NOX's address is nonos_nox_config's, through the chain table (B7). */
    let nox = &crate::wallet::chain::SEPOLIA.nox;
    let nox = crate::wallet::screen::receive_address::checksummed_text(nox);
    for (name, value) in [
        ("pool", POOL),
        ("verifier", VERIFIER),
        ("association registry", REGISTRY),
        ("amount policy", POLICY),
        ("NOX token", nox.as_str()),
        ("faucet", FAUCET),
    ] {
        y += GAP / 2 + super::pick::label(fb, c, y + GAP / 2, &name.to_uppercase());
        y += value_block(fb, c.x, y, c.w, value);
    }
    y
}
