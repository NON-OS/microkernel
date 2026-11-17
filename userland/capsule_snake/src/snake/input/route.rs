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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use crate::snake::state::Game;

use super::{click, key};

/// Keys and clicks as before; moves and releases go to the hover first, so a
/// move repaints only when the control under the pointer changes.
pub fn on_event(game: &mut Game, event: InputEvent) -> EventOutcome {
    if let Some(out) = game.hover.on_pointer(game.screen, &event) {
        return out;
    }
    match event.kind {
        InputKind::KeyDown => key::on_key(game, event.code),
        InputKind::ButtonDown => press(game, event.x, event.y),
        _ => EventOutcome::Idle,
    }
}

/// A click does what it did before. The press is held on the control it began
/// on until the release, and the control under the pointer is named again,
/// since the click may have changed the screen.
fn press(game: &mut Game, x: i32, y: i32) -> EventOutcome {
    game.hover.pointer.pressed(x, y);
    let out = click::on_click(game, x, y);
    game.hover.rehover(game.screen);
    if out == EventOutcome::Idle && game.hover.target.is_none() {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}
