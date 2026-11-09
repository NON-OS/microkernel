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

use crate::browser::shell::kept::MAX_SIZE;
use crate::browser::shell::{store, Page, Shell};
use crate::browser::state::State;
use crate::browser::ui::reader_hit::Control;

/// A press at content-local `at` on the reader page, fed to the controls
/// the settings card's last paint laid out, so it acts on what was drawn
/// there and a disabled step takes nothing. A control acts and saves the
/// settings; a click on the article only takes the keyboard from the
/// address pill.
pub(super) fn click(state: &mut State, shell: &mut Shell, at: &InputEvent) -> EventOutcome {
    let Step::Act(ctl) = feed(&mut state.ui.reader, at) else {
        return EventOutcome::Idle;
    };
    let r = &mut shell.reader;
    match ctl {
        Control::Close => {
            shell.show(Page::Web);
            return EventOutcome::Repaint;
        }
        Control::Theme(i) => r.theme = i,
        Control::Smaller => r.size = r.size.saturating_sub(1),
        Control::Larger => r.size = (r.size + 1).min(MAX_SIZE),
        Control::Width(i) => r.width = i,
        Control::Time => r.show_time = !r.show_time,
    }
    store::sync(shell);
    EventOutcome::Repaint
}
