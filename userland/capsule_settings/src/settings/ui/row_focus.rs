/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The highlight on the row the keyboard is on.

use nonos_app_skeleton::PaintBuffer;

use super::metrics::CARD_RADIUS;
use super::theme::{FOCUS_RING, ROW_HOVER_BG};

pub fn paint(fb: &mut PaintBuffer, card_x: u32, card_w: u32, screen_y: i32, row_h: u32) {
    if screen_y < 0 {
        return;
    }
    let y = screen_y as u32;
    fb.blend_rect(card_x + 1, y, card_w - 2, row_h, ROW_HOVER_BG);
    fb.stroke_round(card_x + 1, y, card_w - 2, row_h, CARD_RADIUS / 2, 1, FOCUS_RING);
}
