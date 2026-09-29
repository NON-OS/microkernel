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

/* Which properties inherit, and the initial values a keyword on a
 * longhand puts back after its shorthand. */

pub(super) fn inherits(name: &str) -> bool {
    const LIST: &str = "color font line-height letter-spacing word-spacing text-align \
        text-indent text-transform white-space visibility direction cursor quotes tab-size";
    name.starts_with("font-")
        || name.starts_with("list-style")
        || LIST.split(' ').any(|n| n == name)
}

/* The initial value of a longhand that does not inherit, applied where a
 * keyword on it follows a shorthand that set it: the shorthand stays, for
 * its other longhands, and this puts the one back. */
pub(super) fn initial(name: &str) -> Option<&'static str> {
    const TABLE: &str = "flex-basis:auto flex-grow:0 flex-shrink:1 margin-top:0 \
        margin-right:0 margin-bottom:0 margin-left:0 padding-top:0 padding-right:0 \
        padding-bottom:0 padding-left:0 background-color:transparent background-image:none \
        grid-row-start:auto grid-row-end:auto grid-column-start:auto grid-column-end:auto \
        overflow-x:visible overflow-y:visible";
    TABLE
        .split_ascii_whitespace()
        .filter_map(|e| e.split_once(':'))
        .find(|e| e.0 == name)
        .map(|e| e.1)
}
