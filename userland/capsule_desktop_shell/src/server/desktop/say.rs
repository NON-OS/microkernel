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

//! A desktop action the file service refused, said in a toast with its
//! reason (`state::says::vfs`), so an icon that does not change is explained.

use nonos_app_skeleton::log_line::{say as log, Line};

use crate::state::says::{named, vfs};
use crate::state::toast::TOAST_TEXT_MAX;
use crate::state::{Context, NotifyLevel};

pub(super) fn refused(ctx: &mut Context, what: &[u8], code: i32) {
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = named(what, vfs(code), &mut line);
    ctx.toasts.push(&line[..n], NotifyLevel::Error, crate::server::toast_clock::now());
    /* The words and the file service's code; never the file's name. */
    let _ = log(&Line::new(b"SHELL")
        .text(b"desktop: ")
        .text(what)
        .text(vfs(code))
        .text(b" (vfs ")
        .num(code.into())
        .text(b")"));
}
