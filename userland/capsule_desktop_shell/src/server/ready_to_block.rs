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

use crate::state::Context;

/// A launch still loading is looked for every few tens of milliseconds
/// (`handlers/installed_launch_poll.rs`), and the app it brings up waits on
/// the shell's focus frame before it draws, so the serve loop does not park a
/// whole second at a time while one is followed.
pub fn ready_to_block(ctx: &Context) -> bool {
    ctx.input_ready && ctx.wm_notify_ready && ctx.launch.is_none()
}
