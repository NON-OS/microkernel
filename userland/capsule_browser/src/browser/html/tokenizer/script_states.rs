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

/// The script data states (13.2.5.4 and 13.2.5.15 to 13.2.5.31) that decide
/// where a script ends. What they emit is always the input itself, so only
/// the state is tracked.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum S {
    Data,
    Lt,
    EscStart,
    EscStartDash,
    Esc,
    EscDash,
    EscDashDash,
    EscLt,
    DblStart,
    Dbl,
    DblDash,
    DblDashDash,
    DblLt,
    DblEnd,
}
