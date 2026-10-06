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

/* Tables: separated borders two pixels apart, cells padded by one pixel
 * with no border (a border comes from the table's border attribute, a
 * presentational hint), row groups centring their cells vertically. */
pub(super) const SHEET: &str = concat!(
    "table{display:table;border-spacing:2px;border-collapse:separate;box-sizing:border-box}",
    "caption{display:table-caption;text-align:center}",
    "colgroup,col{display:none}",
    "thead,tbody,tfoot{display:table-row-group;vertical-align:middle}",
    "tr{display:table-row}",
    "td,th{display:table-cell;padding:1px}",
    "th{text-align:center}",
);
