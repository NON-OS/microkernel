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
//! The router's notice that a Bluetooth device paired without
//! authentication was refused on the secure path (G-76): the user is told,
//! over the lock screen or prompt it was refused at, to pair it again. The
//! router says it at most once a minute; a notice still up is not repeated.
//! Which frame is the notice: state/router_notice.rs.

use crate::render::sync_toast_layer;
use crate::state::{Context, NotifyLevel};

const SAY: &[u8] = b"Bluetooth input refused: re-pair with a passkey";

pub fn say(ctx: &mut Context) {
    // Held, so a user who looked away still finds it; a press dismisses it.
    ctx.toasts.push_held(SAY, NotifyLevel::Warn, crate::server::toast_clock::now());
    sync_toast_layer(ctx);
}
