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

//! Hold the pointer for the duration of an icon drag. Without the grab, the
//! release routes to whatever window the cursor happens to be over, the shell
//! never sees BUTTON_UP, and the icon is left glued to the cursor.

use crate::state::Context;

// The drag's pointer grab is part of what the shell holds (server/grabs.rs):
// these take it up and put it down right where the drag starts and ends.
pub fn grab_drag(ctx: &mut Context) {
    crate::server::grabs::sync(ctx);
}

pub fn release_drag(ctx: &mut Context) {
    crate::server::grabs::sync(ctx);
}
