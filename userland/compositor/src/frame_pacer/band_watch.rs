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

//! The band rule (`band_rule`) applied at the start of a frame. What it was
//! last drawn showing is one word here: the band has one place and one
//! state, the frame is its only reader, and a restarted compositor starts
//! from a full frame anyway.

use core::sync::atomic::{AtomicU32, Ordering};

use super::band_rule::{due, shows};
use super::indicator_band::band;
use crate::state::indicator::{KIND_CAMERA, KIND_MICROPHONE};
use crate::state::Context;

static DRAWN: AtomicU32 = AtomicU32::new(0);

pub fn watch(ctx: &mut Context) {
    let trusted = crate::state::trusted_rule::holder(ctx.scene.layers()).is_some();
    let (camera, microphone) =
        (ctx.indicators.shown(KIND_CAMERA), ctx.indicators.shown(KIND_MICROPHONE));
    let now = shows(camera, microphone, ctx.screen_notice_until != 0, trusted);
    // Damaged now, drawn this frame or kept damaged until a present takes it.
    if due(DRAWN.swap(now, Ordering::Relaxed), now) {
        ctx.damage.accumulate(band(ctx.width, ctx.height));
    }
}
