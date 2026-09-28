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
 * The presses every Shield screen has in the same place: back in the bar,
 * and the dismiss control on whichever banner is up.
 */

use crate::wallet::etna::frame_spec::FrameLayout;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::State;

pub fn put(state: &State, l: &FrameLayout) {
    if let Some(back) = l.back {
        hits::put(Press::Back, back);
    }
    if let Some(d) = l.dismiss {
        hits::put(Press::Dismiss, d);
    }
}

/* The footer buttons that are enabled, as Footer(n) presses. */
pub fn footer(l: &FrameLayout, enabled: &[bool]) {
    for (i, on) in enabled.iter().enumerate() {
        if *on {
            hits::put(Press::Footer(i as u8), l.footer[i]);
        }
    }
}
