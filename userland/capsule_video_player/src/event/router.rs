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

use super::action::Action;
use super::hit::click;
use super::key::{from_key, from_library_key};
use crate::app::state::VideoApp;
use crate::ui::view::page_geom::has_tools;

fn action_for(app: &VideoApp, event: InputEvent) -> Action {
    let chrome = app.route().chrome();
    match event.kind {
        InputKind::KeyDown if chrome => match from_library_key(event.code) {
            // Typing goes to the search field, which only the list pages show.
            Action::Type(_) | Action::Erase if !has_tools(app.route()) => Action::None,
            action => action,
        },
        InputKind::KeyDown => from_key(event.code),
        InputKind::ButtonDown => click(app, event.x, event.y),
        InputKind::Wheel if chrome => Action::Scroll(event.delta_y),
        _ => Action::None,
    }
}

pub fn on_event(app: &mut VideoApp, event: InputEvent) -> EventOutcome {
    if let Some(out) = app.on_pointer(&event) {
        return out;
    }
    // A store that could not be listed is asked again on the next key or click,
    // by the tick that scans: never per pointer move, never in a loop.
    let press = matches!(event.kind, InputKind::KeyDown | InputKind::ButtonDown);
    if press && app.browse.scan_error.is_some() {
        app.browse.scanned = false;
    }
    if event.kind == InputKind::ButtonDown {
        app.hover.pointer.pressed(event.x, event.y);
    }
    let action = action_for(app, event);
    let out = super::apply::apply(app, action);
    app.settled(out, event.kind == InputKind::ButtonDown)
}
