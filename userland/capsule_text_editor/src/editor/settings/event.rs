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

//! Input routing for the Settings screen. Presses land on the rail rows and on
//! the switches of the selected section; a click that misses both stays idle.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::super::app::Editor;
use super::geom::{nav_rect, NAV_LABELS, NAV_PX};
use super::sect::section;
use super::sect_event::section_press;
use super::state::{select_nav, state};
use crate::editor::widget::navlist_hit;

pub(crate) fn settings_event(_ed: &mut Editor, event: InputEvent) -> EventOutcome {
    if !matches!(event.kind, InputKind::ButtonDown) {
        return EventOutcome::Idle;
    }
    if let Some(i) = navlist_hit(nav_rect(), NAV_LABELS.len(), NAV_PX, event.x, event.y) {
        return if select_nav(i) { EventOutcome::Repaint } else { EventOutcome::Idle };
    }
    let nav = state().nav;
    match section(nav) {
        Some(sec) => section_press(nav, sec, event.x, event.y),
        None => EventOutcome::Idle,
    }
}
