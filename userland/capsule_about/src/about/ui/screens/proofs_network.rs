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

//! Every admitted capsule that holds Network: the complete list of programs on
//! this machine that can send a packet anywhere. Each with the measurement its
//! proof was checked against and whose proof admitted it. Anything not listed
//! here cannot reach the network through any service, because every network
//! service refuses a caller without Network.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left};

use crate::about::data::proofs::Snapshot;
use crate::about::data::verify::Attested;

use super::super::card::{self, titled};
use super::super::kv::row_h;
use super::super::metrics::metrics;
use super::verify_spawn_row::row;

pub fn height(s: &Snapshot) -> u32 {
    let (m, rh) = (metrics(), row_h());
    card::overhead() + rh + rh * (s.holders.len().max(1) as u32) + m.card_foot
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32, s: &Snapshot) {
    let (m, rh) = (metrics(), row_h());
    let r = roles();
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(s), b"Can reach the network");
    let note = "The only programs that can send a packet. Each was proved before it started.";
    left(fb, m.card_pad, top + centre_top(0, rh, m.body_px), note, r.muted, m.body_px);
    let first = top + rh as i32;
    if s.census.is_none() {
        let msg = "the kernel did not list them: reading the registry needs AttestRead";
        left(fb, m.card_pad, first + centre_top(0, rh, m.body_px), msg, r.muted, m.body_px);
        return;
    }
    if s.holders.is_empty() {
        let msg = "no running capsule holds Network";
        left(fb, m.card_pad, first + centre_top(0, rh, m.body_px), msg, r.muted, m.body_px);
        return;
    }
    for (i, h) in s.holders.iter().enumerate() {
        let a = Attested {
            pid: h.pid,
            measurement: h.measurement,
            authority: h.authority,
            name: h.name,
            name_len: h.name_len,
        };
        row(fb, m.card_pad, first + (i as u32 * rh) as i32, inner, &a);
    }
}
