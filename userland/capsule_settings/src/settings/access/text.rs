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

//! What each row says, as a person reads it.

use super::knobs::AccessPage;
use crate::settings::schema::access_knob::Knob;

const DISPLAYS: [&str; 4] = ["Display 1", "Display 2", "Display 3", "Display 4"];
const FILTERS: [&str; 5] = ["None", "Greyscale", "Red-weak", "Green-weak", "Blue-weak"];

pub fn value(page: &AccessPage, knob: Knob) -> &'static str {
    let k = &page.knobs;
    if !page.read && knob != Knob::Output {
        return "not read";
    }
    match knob {
        Knob::Output => DISPLAYS.get(page.output as usize).copied().unwrap_or("?"),
        Knob::Filter => FILTERS.get(k.filter as usize).copied().unwrap_or("?"),
        Knob::Contrast => ["Off", "On"][(k.contrast == 1) as usize],
        Knob::Follow => ["Pointer", "Focused window"][(k.follow == 1) as usize],
        Knob::Magnify => match k.magnify {
            6 => "1.5x",
            8 => "2x",
            12 => "3x",
            16 => "4x",
            24 => "6x",
            32 => "8x",
            _ => "Off",
        },
    }
}
