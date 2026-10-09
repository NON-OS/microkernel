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

//! Flip Spotlight and repaint. Shared by the IPC open request and the menu
//! bar's magnifier so both leave the screen in the same state.

//! The menubar's magnifier and the Spotlight request (OP_SPOTLIGHT_OPEN)
//! open the Launchpad, whose search filters every app, tool and installed
//! program as it is typed, or close it when it is already up.
//!
//! They used to show a blank panel with no field and no results, drawn over
//! nothing, that only the same magnifier put away.

use crate::state::Context;

pub fn toggle(ctx: &mut Context) {
    if ctx.launchpad {
        super::launchpad::close(ctx);
    } else {
        super::launchpad::open(ctx);
    }
}
