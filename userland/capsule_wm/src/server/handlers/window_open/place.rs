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

//! Place a newly opened window.
//!
//! Centred, then cascaded. A window opens in the middle of the screen, which is
//! where a person is looking and where every desktop they have used puts one.
//! Successive windows step down and right from that centre so their titlebars
//! stay reachable and clicking one remains a reliable way to switch, which is
//! what the cascade was always for.
//!
//! It used to cascade from a fixed top-left corner instead, so the first window
//! of a session opened in the upper left whatever its size, and each
//! application also carried its own hand-picked origin that this then ignored.
//! Two sources of position, neither of them the middle.
//!
//! Non-normal windows (dialogs, tooltips) keep their requested position.

use crate::geometry::{clamp_to_display, Rect};
use crate::state::Context;
use crate::window::{Kind, Visibility};

use super::cascade::cascade;

pub(super) fn place(ctx: &Context, kind: Kind, requested: Rect) -> Rect {
    let requested = clamp_to_display(requested, ctx.display_width, ctx.display_height);
    if kind != Kind::Normal {
        return requested;
    }
    let open = ctx
        .windows
        .windows()
        .filter(|w| w.kind == Kind::Normal && w.visibility == Visibility::Visible)
        .count() as u32;
    cascade(ctx.display_width, ctx.display_height, open, requested)
}
