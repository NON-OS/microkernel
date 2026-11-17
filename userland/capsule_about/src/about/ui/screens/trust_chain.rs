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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::left;

use crate::about::data::trust;

use super::super::card::{self, titled};
use super::super::kv::{kv, row_h};
use super::super::metrics::metrics;
use super::trust_hops::nodes;
use super::{chain, prose};

const CHAIN_GAP: u32 = 14;
const EVIDENCE_GAP: u32 = 12;

pub fn height(inner: u32) -> u32 {
    let (m, rh) = (metrics(), row_h());
    let chain_gap = scale().px(CHAIN_GAP);
    let ev_gap = scale().px(EVIDENCE_GAP);
    let evidence = m.pair_line + prose::height(trust::STATUS, inner);
    card::overhead() + m.chain_h + chain_gap + rh * 3 + ev_gap + evidence
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (m, rh) = (metrics(), row_h());
    let chain_gap = scale().px(CHAIN_GAP);
    let ev_gap = scale().px(EVIDENCE_GAP);
    let r = roles();
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(inner), b"Signing chain");
    let mut hops: [&'static [u8]; 4] = [b""; 4];
    let count = nodes(&mut hops);
    chain::paint(fb, m.card_pad, top, &hops[..count]);
    let rows: [(&[u8], &[u8]); 3] = [
        (b"Scheme", trust::HYBRID_SCHEME),
        (b"Manifest", trust::MANIFEST_FORMAT),
        (b"Certificate", trust::CERT_FORMAT),
    ];
    let first = top + (m.chain_h + chain_gap) as i32;
    for (i, (label, value)) in rows.into_iter().enumerate() {
        kv(fb, m.card_pad, first + (i as u32 * rh) as i32, inner, label, value, false);
    }
    let ev = first + (rh * 3 + ev_gap) as i32;
    left(fb, m.card_pad, ev, "Evidence", r.muted, m.body_px);
    prose::paint(fb, m.card_pad, ev + m.pair_line as i32, inner, trust::STATUS, r.text_2);
}
