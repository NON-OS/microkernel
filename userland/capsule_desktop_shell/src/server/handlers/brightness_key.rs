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

//! A brightness key: the compositor steps the focused output's panel
//! (state/brightness.rs says what came of it), and the notice replaces the
//! last brightness notice rather than stacking under it.

use crate::compositor_client::brightness::step;
use crate::state::brightness::{heard, is_brightness_notice, notice};
use crate::state::toast::TOAST_TEXT_MAX;
use crate::state::{Context, NotifyLevel};

/// Brightness Up (`up`) or Down, on its key-down.
pub fn key(ctx: &mut Context, up: bool) {
    let rid = ctx.issue_request_id();
    let lit = heard(step(ctx.ports.compositor_port, rid, up));
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = notice(lit, &mut line);
    let now = crate::server::toast_clock::now();
    ctx.toasts.replace(&line[..n], NotifyLevel::Info, now, is_brightness_notice);
}
