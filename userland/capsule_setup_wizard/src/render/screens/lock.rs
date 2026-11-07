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

/*
 * The lock screen's passphrase: typed, then typed again, shown as stars.
 * Not asked when installing: the installed system asks on its first start.
 */

use nonos_lock::rules::Refusal;

use crate::lock::{Keyed, Stage};
use crate::render;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, rows, text::cat};
use crate::server::step::Outcome;
use crate::state::Context;

const RULES: &[&[u8]] = &[
    b"The screen locks after the idle time set in Settings, on Super+L,",
    b"and when the lid closes. This passphrase is asked for then.",
    b"At least 8 characters. Only a slow hash of it is kept.",
    b"There is no way to recover a forgotten passphrase.",
];

pub fn draw(ctx: &Context) {
    let (title, keys): (&[u8], &[u8]) = match ctx.lock.stage {
        Stage::New => (b"Type a passphrase for the lock screen.", b"TYPE  ENTER NEXT  ESC BACK"),
        Stage::Again => (b"Type the same passphrase again.", b"TYPE  ENTER NEXT  ESC RETYPE"),
        Stage::Chosen => (
            b"Chosen. It is saved when setup finishes.",
            b"ENTER NEXT  BACKSPACE CHOOSE ANOTHER  ESC BACK",
        ),
    };
    render::frame(ctx, b"Lock screen", title, keys);
    let l = render::layout_of(ctx);
    let (cv, x) = (&mut render::canvas(ctx), l.col_x);
    let typed = match ctx.lock.stage {
        Stage::New => ctx.lock.new.chars(),
        Stage::Again => ctx.lock.again.chars(),
        Stage::Chosen => 0,
    };
    let mut stars = [b'*'; 64];
    let shown = typed.min(stars.len() - 1);
    stars[shown] = b'_';
    let y = rows::list(cv, x, l.body_y, &[&stars[..=shown]], 0);
    let y = lines::text(cv, x, y + l.gap, RULES, HINT);
    let said: &[u8] = match ctx.lock.refused {
        Some(Refusal::Short) => b"At least 8 characters, and not only spaces.",
        Some(Refusal::Long) => b"At most 256 bytes.",
        Some(Refusal::NotText) => b"Use letters, digits, spaces and punctuation.",
        Some(Refusal::Mismatch) => b"The two entries differ. Type the passphrase again.",
        None => return,
    };
    let mut line = [0u8; 80];
    lines::text(cv, x, y + l.gap, &[cat(&mut line, &[said])], FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match ctx.lock.key(code) {
        Keyed::Advance => Outcome::Advance,
        Keyed::Back => Outcome::Back,
        Keyed::Stay => Outcome::Stay,
    }
}
