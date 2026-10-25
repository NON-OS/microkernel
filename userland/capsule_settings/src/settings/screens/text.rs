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

//! What each of the displays' rows says, as a person reads it.

use super::arrange::ScreenPage;
use super::percent::percent_of;
use crate::settings::schema::access_knob::Knob;

const DISPLAYS: [&str; 4] = ["Display 1", "Display 2", "Display 3", "Display 4"];

fn percent(q: u32) -> &'static str {
    match q {
        4 => "100%",
        5 => "125%",
        6 => "150%",
        8 => "200%",
        12 => "300%",
        16 => "400%",
        _ => "?",
    }
}

fn automatic(q: u32) -> &'static str {
    match q {
        4 => "Automatic, 100%",
        5 => "Automatic, 125%",
        6 => "Automatic, 150%",
        8 => "Automatic, 200%",
        12 => "Automatic, 300%",
        16 => "Automatic, 400%",
        _ => "Automatic",
    }
}

pub fn value(page: &ScreenPage, knob: Knob) -> &'static str {
    let s = &page.shown;
    if !page.read {
        return "not read";
    }
    match knob {
        Knob::Arrange if s.extend && !s.extended => "Extended, on a second display",
        Knob::Arrange => ["Mirrored", "Extended"][s.extend as usize],
        Knob::Screen => DISPLAYS.get(page.output as usize).copied().unwrap_or("?"),
        Knob::Scale => match s.chosen.get(page.output as usize).copied().unwrap_or(0) {
            // Only the first display's scale now is told; it is the canvas's.
            0 if page.output == 0 => automatic(s.scale_q),
            0 => "Automatic",
            q => percent(q),
        },
        Knob::Brightness => match page.levels.get(page.output as usize).copied().flatten() {
            Some(l) => percent_of(l),
            None => "not offered",
        },
        _ => "",
    }
}
