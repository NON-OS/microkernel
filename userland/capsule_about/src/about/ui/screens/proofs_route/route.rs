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

//! The route: what the system is set to, and each transport's own account,
//! with its age on the board's clock. A transport's row gets the verdict its
//! report would earn if the system were set to it, so both are judged by the
//! same rule while only the chosen one decides the headline.

use nonos_app_skeleton::PaintBuffer;
use nonos_route_proof::Network;

use crate::about::data::proofs::{words, Snapshot};
use crate::about::ui::card::{self, titled};
use crate::about::ui::kv::ROW_H;
use crate::about::ui::metrics::CARD_PAD;

use super::super::proofs_verdict::of_route;
use super::super::verify_row::row;
use super::transport::transport;

const PER_TRANSPORT: u32 = 3;

pub const HEIGHT: u32 = card::OVERHEAD + ROW_H + ROW_H * PER_TRANSPORT * 2 + 4;

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32, s: &Snapshot) {
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, HEIGHT, b"Route");
    row(
        fb,
        CARD_PAD,
        top,
        inner,
        of_route(s.route),
        set_to(s.chosen),
        words::route_evidence(s.route),
    );
    let (nym, anyone) = match s.board {
        Some((n, a)) => (n, a),
        None => (None, None),
    };
    let first = top + ROW_H as i32;
    transport(fb, first, inner, Network::Nym, nym, s.board.is_some());
    let second = first + (ROW_H * PER_TRANSPORT) as i32;
    transport(fb, second, inner, Network::Anyone, anyone, s.board.is_some());
}

fn set_to(chosen: Option<u8>) -> &'static [u8] {
    match chosen {
        Some(nonos_route_proof::ROUTE_NYM) => b"this system's traffic is set to the Nym mixnet",
        Some(nonos_route_proof::ROUTE_ANYONE) => {
            b"this system's traffic is set to the Anyone network"
        }
        Some(nonos_route_proof::ROUTE_DIRECT) => b"this system's traffic is set to go direct",
        _ => b"the route this system is set to could not be read",
    }
}
