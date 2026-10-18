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

/* An app's toast line, the app named as its tile names it. */

use crate::state::says::named;
use crate::state::toast::{UptimeMs, TOAST_TEXT_MAX};
use crate::state::{Context, NotifyLevel, LAUNCHER_APPS};

/* The app as its tile names it. */
pub(super) fn label(service: &[u8]) -> &'static [u8] {
    LAUNCHER_APPS.iter().find(|a| a.service == service).map_or(&b"This app"[..], |a| a.label)
}

/* "<App> turned off at setup" or "<App> did not open: why", the app named
 * as its tile names it. */
pub(super) fn say(
    ctx: &mut Context,
    service: &[u8],
    tail: &[u8],
    level: NotifyLevel,
    now: UptimeMs,
) {
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = named(label(service), tail, &mut line);
    ctx.toasts.push(&line[..n], level, now);
}
