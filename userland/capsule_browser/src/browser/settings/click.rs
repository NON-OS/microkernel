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

use nonos_app_skeleton::{feed, EventOutcome, InputEvent};
use nonos_toolkit::widgets::Step;

use crate::browser::paint::pages::extent::page_scroll;
use crate::browser::paint::settings::field;
use crate::browser::shell::Shell;
use crate::browser::tabs::Tabs;
use crate::browser::ui::settings_geom::{Control, Field, SettingsGeom};

/// A press at content-local `at` on the settings page, fed to the controls
/// its last paint laid out, so it acts on what was drawn there and a
/// disabled button takes nothing. Anything but a field takes the keyboard
/// from the fields, and anything but Remove all bookmarks disarms it.
pub fn click(tabs: &mut Tabs, shell: &mut Shell, at: &InputEvent) -> EventOutcome {
    let hit = match feed(&mut tabs.active_mut().ui.settings, at) {
        Step::Act(c) => Some(c),
        _ => None,
    };
    if hit != Some(Control::RemoveBookmarks) {
        shell.settings.armed = false;
    }
    match hit {
        Some(Control::Field(f)) => {
            let c = tabs.active().layout.content;
            let g = SettingsGeom::new(c.w, c.h, page_scroll(shell, c.w, c.h));
            caret_at(shell, f, &g, at.x);
            shell.settings.focus = Some(f);
        }
        Some(ctl) => {
            shell.settings.focus = None;
            super::act::act(tabs, shell, ctl);
        }
        None => shell.settings.focus = None,
    }
    EventOutcome::Repaint
}

/// Puts the caret of field `f` at the character boundary nearest to a
/// click at content-local `x`, where the field was drawn before the click.
fn caret_at(shell: &mut Shell, f: Field, g: &SettingsGeom, x: i32) {
    let Some(r) = g.rect(Control::Field(f)) else { return };
    let at = field::caret_at(&shell.settings, f, r, x);
    let ed = shell.settings.field(f);
    (ed.caret, ed.anchor) = (at, at);
}
