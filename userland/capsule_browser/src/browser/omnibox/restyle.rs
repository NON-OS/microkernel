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

/* Element states a selector can test, as the style cache reports them in
 * its state mask. */
pub const HOVER: u8 = 1;
pub const FOCUS: u8 = 2;
pub const ACTIVE: u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Restyle {
    Nothing,
    /* The browser draws something for the state itself (a focused field). */
    Repaint,
    /* A stylesheet rule tests the state: the page has to be restyled. */
    Relayout,
}

/* What a change of the `changed` element states costs, given the `mask` of
 * states some author rule tests. Hovering over a page whose sheets never
 * say :hover restyles nothing. */
pub fn state_restyle(changed: u8, mask: u8) -> Restyle {
    if changed & mask != 0 {
        Restyle::Relayout
    } else if changed & FOCUS != 0 {
        Restyle::Repaint
    } else {
        Restyle::Nothing
    }
}
