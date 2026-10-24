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

//! The idle heal: a layer whose surface has not attached is damaged again
//! now and then, since no other damage brings it back.

use crate::state::visible_drop::missed;
use crate::state::Context;

// How often a layer that has not attached is tried again when nothing else
// composes. Everything the compositor changes damages what it changed (a
// layer placed, moved, raised or removed, the cursor, the looks, the band,
// a new canvas or a lost output), so an idle desktop composes and presents
// nothing, where this was a full-screen recomposite and present every four
// seconds even at rest. A layer whose surface did not attach is the one
// thing no damage brings back: its rectangle is damaged here, and drawn
// whole once it attaches (state/visible.rs). On the uptime clock, so a
// pointer posting events every few milliseconds does not bring it sooner.
pub const HEAL_INTERVAL_MS: i64 = 4000;

pub fn heal_missed(ctx: &mut Context) {
    missed(&ctx.scene).for_each(|r| ctx.damage.accumulate(r));
}
