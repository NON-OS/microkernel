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

use super::script_states::S;
use super::script_temp::Temp;
use super::state::is_ws;

/// One input byte: the next state and whether the byte was consumed (false
/// means it is read again in the next state). "</" that is not the
/// appropriate end tag reads on as text; the caller checks for the one that
/// is before calling this.
pub fn step(st: S, c: u8, temp: &mut Temp) -> (S, bool) {
    use S::*;
    let name_end = is_ws(c) || c == b'/' || c == b'>';
    let alpha = c.is_ascii_alphabetic();
    match (st, c) {
        (Data, b'<') => (Lt, true),
        (Lt, b'!') => (EscStart, true),
        (Lt, b'/') | (Data, _) => (Data, true),
        (EscStart, b'-') => (EscStartDash, true),
        (EscStartDash, b'-') | (EscDash, b'-') | (EscDashDash, b'-') => (EscDashDash, true),
        (Lt | EscStart | EscStartDash, _) => (Data, false),
        (Esc, b'-') => (EscDash, true),
        (Esc | EscDash | EscDashDash, b'<') => (EscLt, true),
        (EscDashDash | DblDashDash, b'>') => (Data, true),
        (EscLt, b'/') | (Esc | EscDash | EscDashDash, _) => (Esc, true),
        (EscLt, _) if alpha => {
            temp.clear();
            (DblStart, false)
        }
        (DblStart, _) if name_end => (if temp.is_script() { Dbl } else { Esc }, true),
        (DblEnd, _) if name_end => (if temp.is_script() { Esc } else { Dbl }, true),
        (DblStart | DblEnd, _) if alpha => {
            temp.push(c);
            (st, true)
        }
        (EscLt | DblStart, _) => (Esc, false),
        (Dbl, b'-') => (DblDash, true),
        (DblDash | DblDashDash, b'-') => (DblDashDash, true),
        (Dbl | DblDash | DblDashDash, b'<') => (DblLt, true),
        (Dbl | DblDash | DblDashDash, _) => (Dbl, true),
        (DblLt, b'/') => {
            temp.clear();
            (DblEnd, true)
        }
        (DblLt | DblEnd, _) => (Dbl, false),
    }
}
