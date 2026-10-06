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

use crate::browser::dom::node::Node;

use super::form_kind::input_kind;

/* A number as HTML's floating-point rules read one; not a finite number is
 * no number. */
pub(super) fn num(v: Option<&str>) -> Option<f64> {
    v?.parse::<f64>().ok().filter(|f| f.is_finite())
}

/* Some(true) in range, Some(false) out of range, None for an element with
 * no range limitations (HTML 4.10.5): an input of type number or one of
 * the date and time types with a min or a max, judged on the value in its
 * markup, and a range input, whose value is always clamped into range. An
 * empty or unreadable value is in range. Date and time values compare as
 * the fixed-width ISO strings they are; a bound written with a different
 * precision than the value (09:30 against 09:30:15) is not compared. */
pub(super) fn range_state(n: &Node) -> Option<bool> {
    if n.tag != "input" {
        return None;
    }
    let (min, max, value) = (n.attr("min"), n.attr("max"), n.attr("value").unwrap_or(""));
    let limited = min.is_some() || max.is_some();
    match input_kind(n) {
        "range" => Some(true),
        "number" if limited => {
            let Some(v) = num(Some(value)) else { return Some(true) };
            Some(!num(min).is_some_and(|m| v < m) && !num(max).is_some_and(|m| v > m))
        }
        "date" | "month" | "week" | "time" | "datetime-local" if limited => {
            let same = |b: &&str| !value.is_empty() && b.len() == value.len();
            let below = min.filter(same).is_some_and(|m| value < m);
            let above = max.filter(same).is_some_and(|m| value > m);
            Some(!below && !above)
        }
        _ => None,
    }
}

/* A number input whose value is off its step: with a min to count from and
 * a step other than "any" (1 by default), value - min is not a whole number
 * of steps. */
pub(super) fn step_mismatch(n: &Node) -> bool {
    let (Some(v), Some(base)) = (num(n.attr("value")), num(n.attr("min"))) else {
        return false;
    };
    let step = match n.attr("step") {
        Some(s) if s.eq_ignore_ascii_case("any") => return false,
        Some(s) => num(Some(s)).filter(|s| *s > 0.0).unwrap_or(1.0),
        None => 1.0,
    };
    let k = (v - base) / step;
    /* Nearest whole step, by hand: f64::round needs std. */
    let whole = if k < 0.0 { (k - 0.5) as i64 } else { (k + 0.5) as i64 };
    (k - whole as f64).abs() > 1e-9
}
