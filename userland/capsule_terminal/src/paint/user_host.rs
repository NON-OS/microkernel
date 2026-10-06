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

use super::line_text::text;
use crate::term::identity::{hostname, username};
use crate::term::theme::types::Theme;

/*
 * `user@host ` ahead of the path, in the text colour, the way zsh on a Mac
 * opens its prompt; returns the cells it took. The path's tail is what says
 * where you are, so on a line with no room for both the name gives way and
 * nothing is drawn.
 */
pub(super) fn draw_id(
    fb: &mut PaintBuffer,
    ox: u32,
    y: u32,
    adv: u32,
    px: f32,
    room: usize,
    t: &Theme,
) -> usize {
    let (user, host) = (username(), hostname());
    let cells = user.len() + 1 + host.len() + 1;
    if cells >= room {
        return 0;
    }
    text(fb, ox, y, user, t.fg, adv, px);
    let at = ox + user.len() as u32 * adv;
    text(fb, at, y, b"@", t.fg, adv, px);
    text(fb, at + adv, y, host, t.fg, adv, px);
    cells
}

/* The splash's `user@host`: the name in the accent, the host dimmed after it. */
pub(super) fn draw_fetch_id(fb: &mut PaintBuffer, x: i32, y: i32, px: f32, t: &Theme) {
    let user = core::str::from_utf8(username()).unwrap_or("nonos");
    let _ = fb.text_ttf_mono(x, y, user, t.accent, px);
    let mut host = [0u8; 40];
    host[0] = b'@';
    let hn = hostname();
    let hl = hn.len().min(host.len() - 1);
    host[1..1 + hl].copy_from_slice(&hn[..hl]);
    let at = core::str::from_utf8(&host[..1 + hl]).unwrap_or("@");
    let gap = fb.measure_ttf_mono(user, px);
    let _ = fb.text_ttf_mono(x + gap, y, at, t.dim, px);
}
