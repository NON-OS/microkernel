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

//! One listing, drawn as a card: the toolkit's `Card` is the box, and the
//! tile, the two lines and the controls are drawn inside it.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Button, Card, Ink, Kind, Label, Pill, Rect, State as Look};

use crate::store::listing::{Listing, Source};
use crate::store::model_card::model_line;
use crate::store::model_weights::MODEL_WEIGHTS;
use crate::status_wire::Status;
use crate::store::progress::Progress;
use crate::store::route_offer::progress_line;
use crate::store::state::State;

use super::bytes::as_str;
use super::geometry::{action_rect, remove_rect};
use super::hover_look::{look, pointed};
use super::metrics::metrics;
use super::target::Target;

/// How far below a card's top its name and the line under it start, in
/// logical pixels.
const NAME_TOP: u32 = 12;
const SUB_TOP: u32 = 34;

/// The card at `at` in the visible list. The keyboard cursor's card draws
/// the focus ring, and the card and its buttons draw the pointer's hover and
/// press.
pub fn paint(
    fb: &mut PaintBuffer,
    l: &Listing,
    fetch: Option<Status>,
    (x, y, w): (u32, u32, u32),
    (state, at): (&State, usize),
) {
    let (r, m, s) = (roles(), metrics(), scale());
    let card = look(state, |t| t.on_card(at), at == state.cursor);
    Card::new().state(card).draw(fb, Rect { x, y, w, h: m.card_h });

    tile(fb, l, x + m.card_pad, y);

    let text_x = x + m.card_pad + m.tile + m.tile_gap;
    text::left(fb, text_x, (y + s.px(NAME_TOP)) as i32, as_str(&l.name), r.text, m.name_px);
    /*
     * A Qwen tier says what it downloads and the memory it needs, and while
     * its model comes, how far it has come (`route_offer::progress_line`).
     */
    let (progress, model) = (fetch.map(|s| progress_line(&s)), model_line(&l.id, MODEL_WEIGHTS));
    let (line, ink) = match (&progress, &model) {
        (Some(p), _) => (p.as_str(), Ink::Accent),
        (None, Some(m)) => (as_str(m), Ink::Muted),
        (None, None) => (as_str(l.source.origin()), Ink::Muted),
    };
    let installed = l.progress == Progress::Installed;
    let controls = if installed { remove_rect(x, y, w).0 } else { action_rect(x, y, w).0 };
    let sub = Label::new(line).ink(ink);
    let room = controls.saturating_sub(text_x + m.card_pad / 2);
    sub.draw(fb, Rect { x: text_x, y: y + s.px(SUB_TOP), w: room, h: sub.size().1 });

    action(fb, l, action_rect(x, y, w), pointed(state, |t| t == Target::Action(at)));
    if installed {
        let (x, y, w, h) = remove_rect(x, y, w);
        let look = pointed(state, |t| t == Target::Remove(at));
        Button::new("Remove").kind(Kind::Danger).state(look).draw(fb, Rect { x, y, w, h });
    }
}

/// The install action, filled rather than lettered where it is the offer:
/// a coloured word is not obviously a control. `look` is the pointer's; a
/// disabled action ignores it.
fn action(fb: &mut PaintBuffer, l: &Listing, (x, y, w, h): (u32, u32, u32, u32), look: Look) {
    let (kind, disabled) = match l.progress {
        p if p.uninstalled() && l.ready => (Kind::Primary, false),
        Progress::Installed => (Kind::Primary, false),
        Progress::Queued | Progress::Installing | Progress::Removing => (Kind::Plain, false),
        p if p.needs_installed_system() => (Kind::Secondary, true),
        p if p.retryable() => (Kind::Danger, false),
        Progress::Refused | Progress::Failed(_) | Progress::Idle | Progress::Removed => {
            (Kind::Secondary, false)
        }
    };
    let look = Look { disabled, ..look };
    let button = Button::new(as_str(l.progress.button(l.ready))).kind(kind).state(look);
    button.draw(fb, Rect { x, y, w, h });
}

/// The listing's initial in a pill centred on the tile at `x`, washed in
/// the accent for a NONOS listing so the namespaces are told apart before a
/// word is read. Linux packages, models and community submissions share the
/// plain pill; the line under the name says which they are.
fn tile(fb: &mut PaintBuffer, l: &Listing, x: u32, y: u32) {
    let initial = [l.name.first().copied().unwrap_or(b'?').to_ascii_uppercase()];
    let own = Look { selected: l.source == Source::NonOs, ..Look::default() };
    let pill = Pill::new(as_str(&initial)).state(own);
    let (w, m) = (pill.size().0, metrics());
    pill.draw(fb, Rect { x: x + m.tile.saturating_sub(w) / 2, y, w, h: m.card_h });
}
