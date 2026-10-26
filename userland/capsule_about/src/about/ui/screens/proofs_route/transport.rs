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

//! One transport's rows on the route card: the verdict its report would earn
//! as the chosen route, then its directory, validity and hops.

use nonos_app_skeleton::PaintBuffer;
use nonos_route_proof::{route_verdict, Latest, Network, ROUTE_ANYONE, ROUTE_NYM};

use crate::about::data::proofs::words;
use crate::about::theme::MUTED;
use crate::about::ui::kv::ROW_H;
use crate::about::ui::metrics::{BODY_PX, CARD_PAD};
use crate::about::ui::text::{line, top_of};

use super::super::proofs_verdict::of_route;
use super::super::verify_row::{row, MARK_W};
use super::join::{join, join3};

pub fn transport(fb: &mut PaintBuffer, y: i32, w: u32, n: Network, latest: Latest, board: bool) {
    let as_chosen = match n {
        Network::Nym => ROUTE_NYM,
        Network::Anyone => ROUTE_ANYONE,
    };
    let v = route_verdict(Some(as_chosen), latest);
    let mut claim = [0u8; 64];
    let stage = latest.map_or(silent(board), |(r, _)| words::stage(r.stage));
    let c = join(&mut claim, words::network(n), b": ", stage);
    let mut age = [0u8; 48];
    let ev: &[u8] = match latest {
        Some((_, ms)) => words::duration(ms, &mut age),
        None => b"",
    };
    row(fb, CARD_PAD, y, w, of_route(v), &claim[..c], ev);
    let Some((r, ms)) = latest else {
        return;
    };
    let x = CARD_PAD + MARK_W;
    let mut d = [0u8; 48];
    let mut left = [0u8; 48];
    let mut line1 = [0u8; 96];
    let valid = words::duration(r.valid_for_ms.saturating_sub(ms), &mut left);
    let n1 = join3(&mut line1, words::directory(&r, &mut d), b", valid for ", valid);
    line(fb, x, top_of(y + ROW_H as i32, ROW_H, BODY_PX), &line1[..n1], MUTED, BODY_PX);
    let mut h = [0u8; 48];
    let mut line2 = [0u8; 96];
    let hops = words::hops(&r, &mut h);
    let n2 = match n {
        Network::Nym if r.cover_traffic => join(&mut line2, hops, b", ", b"cover traffic on"),
        Network::Nym => join(&mut line2, hops, b", ", b"cover traffic off"),
        Network::Anyone => join(&mut line2, hops, b"", b""),
    };
    line(fb, x, top_of(y + 2 * ROW_H as i32, ROW_H, BODY_PX), &line2[..n2], MUTED, BODY_PX);
}

fn silent(board: bool) -> &'static [u8] {
    if board {
        b"has not reported"
    } else {
        b"route board not reachable"
    }
}
