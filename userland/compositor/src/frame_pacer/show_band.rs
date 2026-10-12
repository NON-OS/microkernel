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

//! The indicator band made visible now: damaged and the frame presented at
//! once, from inside the INDICATOR request, so the change is on the screen
//! before the request answers. Also the upkeep's way to show a holder that
//! died being dropped.

use super::indicator_band::band;
use crate::state::Context;

pub fn show_band(ctx: &mut Context) -> Result<(), &'static str> {
    ctx.damage.accumulate(band(ctx.width, ctx.height));
    super::tick(ctx)
}
