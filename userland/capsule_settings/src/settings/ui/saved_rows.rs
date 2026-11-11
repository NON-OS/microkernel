/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One saved network, by name. With none, one row says so or says why the
//! record could not be read.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Rect, Row, Scrolled};

use crate::settings::state::State;

use super::bytes::as_str;
use super::control_geom::right_edge;
use super::hover_look::look;
use super::metrics::metrics;
use super::target::Target;

/// One saved network at `at`, in content coordinates, through the toolkit
/// row; "Connected" stays in the room the row leaves at its right end.
pub fn paint(fb: &mut PaintBuffer, state: &State, index: usize, scroll: u32, at: Rect) {
    let w = &state.wifi;
    if w.saved_count == 0 {
        let why = w.saved_err.map_or("None saved", |e| e.text());
        Row::new(why).draw(&mut Scrolled::new(fb, scroll), at);
        return;
    }
    let i = index.min(w.saved_count - 1);
    let at_i = state.wifi_network_count + i;
    let look = look(state, |t| t == Target::Network(at_i), state.wifi_cursor == at_i);
    let net = w.saved[i];
    let note = if net.secured { "Passphrase sealed" } else { "Open network" };
    let (joined, px) = (w.joined() == Some(net.ssid()), metrics().body_px);
    let room = if joined { text::width("Connected", px) } else { 0 };
    let line = Row::new(as_str(net.ssid())).note(note).trailing(room).state(look);
    line.draw(&mut Scrolled::new(fb, scroll), at);
    if joined {
        let top = text::centre_top(0, at.h, px) + at.y as i32 - scroll as i32;
        text::right(fb, right_edge(at.x, at.w), top, "Connected", roles().ok, px);
    }
}
