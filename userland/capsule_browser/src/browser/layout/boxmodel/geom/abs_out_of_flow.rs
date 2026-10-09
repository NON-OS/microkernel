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

use crate::browser::css::{Computed, Position};

/* An absolutely positioned or fixed box always leaves normal flow. With
 * every inset auto it is placed at its static position, the spot flow
 * would have given it, but it still takes no space there. */
pub(crate) fn out_of_flow(s: &Computed) -> bool {
    s.position == Position::Absolute
}

/* A box that places its absolutely positioned descendants: any position
 * other than static, sticky included, and a transformed box, whose
 * transform then carries those descendants along with it. */
pub(crate) fn positioned(s: &Computed) -> bool {
    s.position != Position::Static || s.is_sticky || s.fx.transform.is_some()
}
