/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One saved network, by name. With none, one row says so or says why the
//! record could not be read.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::state::State;

use super::bytes::as_str;
use super::control_geom::right_edge;
use super::metrics::BODY_PX;
use super::row_focus;
use super::row_label;
use super::text;
use super::theme::OK;

pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    index: usize,
    card_x: u32,
    card_w: u32,
    screen_y: i32,
    row_h: u32,
) {
    let w = &state.wifi;
    if w.saved_count == 0 {
        let why = w.saved_err.map_or("None saved", |e| e.text());
        row_label::paint(fb, card_x, screen_y, row_h, why, None);
        return;
    }
    let i = index.min(w.saved_count - 1);
    if state.wifi_cursor == state.wifi_network_count + i {
        row_focus::paint(fb, card_x, card_w, screen_y, row_h);
    }
    let net = w.saved[i];
    let note = if net.secured { "Passphrase sealed" } else { "Open network" };
    row_label::paint(fb, card_x, screen_y, row_h, as_str(net.ssid()), Some(note));
    if w.joined() == Some(net.ssid()) {
        let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
        text::right(fb, right_edge(card_x, card_w), top, "Connected", OK, BODY_PX);
    }
}
