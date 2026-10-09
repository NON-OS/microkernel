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
use tools_2048::Game;

use super::colors::tile_color;
use super::decimal::{render_u32, DIGITS};

const BG: u32 = 0xFF1A1A2E;
const BOARD_BG: u32 = 0xFF3A3A4E;
const TEXT: u32 = 0xFFF5F5F0;
const ACCENT: u32 = 0xFFEDC22E;

const ORIGIN_X: u32 = 33;
const ORIGIN_Y: u32 = 150;
const TILE: u32 = 66;
const STEP: u32 = 76;

/// The game is laid out at its opening window's size. A larger window
/// (green, full screen) gets the same game centred on the ground, which
/// fills the whole window; it was drawn in the top left corner.
const LAYOUT_W: u32 = super::manifest::WIDTH;
const LAYOUT_H: u32 = super::manifest::HEIGHT;

pub fn paint(fb: &mut PaintBuffer, game: &Game<4>, over: bool) {
    fb.clear(BG);
    let ox = fb.width.saturating_sub(LAYOUT_W) / 2;
    let oy = fb.height.saturating_sub(LAYOUT_H) / 2;
    fb.text_scaled(ox + 24, oy + 34, b"2048", ACCENT, 4);
    fb.text(ox + 210, oy + 40, b"SCORE", TEXT);
    let mut sbuf = [0u8; DIGITS];
    fb.text_scaled(ox + 210, oy + 58, render_u32(game.score() as u32, &mut sbuf), TEXT, 2);
    fb.fill_rect(ox + ORIGIN_X - 9, oy + ORIGIN_Y - 9, STEP * 4, STEP * 4, BOARD_BG);
    let board = game.board();
    for r in 0..4 {
        for c in 0..4 {
            let v = board[r][c];
            let x = ox + ORIGIN_X + c as u32 * STEP;
            let y = oy + ORIGIN_Y + r as u32 * STEP;
            fb.fill_rect(x, y, TILE, TILE, tile_color(v));
            if v != 0 {
                let mut tbuf = [0u8; DIGITS];
                let digits = render_u32(v as u32, &mut tbuf);
                let tx = x + TILE / 2 - digits.len() as u32 * 5;
                fb.text_scaled(tx, y + TILE / 2 - 8, digits, TEXT, 2);
            }
        }
    }
    if over {
        fb.text_scaled(ox + 64, oy + 250, b"GAME OVER", ACCENT, 4);
    }
}
