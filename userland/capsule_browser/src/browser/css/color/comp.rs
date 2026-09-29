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

use super::args::Comp;

/// One token: a number with an optional %, deg, rad, grad or turn unit.
pub(super) fn comp(tok: &str) -> Option<Comp> {
    if tok.eq_ignore_ascii_case("none") {
        return Some(Comp::Missing);
    }
    let split = tok.find(|ch: char| ch.is_ascii_alphabetic() && ch != 'e' && ch != 'E');
    let (num, unit) = match tok.strip_suffix('%') {
        Some(n) => (n, "%"),
        None => tok.split_at(split.unwrap_or(tok.len())),
    };
    let v = num.parse::<f64>().ok().filter(|v| v.is_finite())?;
    if unit == "%" {
        return Some(Comp::Pct(v));
    }
    let deg = 180.0 / core::f64::consts::PI;
    let units = [("", 1.0), ("deg", 1.0), ("rad", deg), ("grad", 0.9), ("turn", 360.0)];
    let k = units.iter().find(|(u, _)| unit.eq_ignore_ascii_case(u))?.1;
    Some(Comp::Num(v * k))
}
