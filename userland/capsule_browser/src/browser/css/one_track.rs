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

use crate::browser::css::computed::GridTrack;

use super::parse_px::parse_px;
use super::strip_unit::strip_unit;

/* One grid track token. The content keywords keep their meaning; a
 * fit-content() track sizes like auto. minmax(a, b) collapses to its max
 * side. The self-call terminates because each strip shortens the token.
 * Fractions and percentages keep two decimals. */
pub(super) fn one_track(tok: &str, em: u32) -> Option<GridTrack> {
    let t = tok.trim();
    let low =
        |p: &str| t.len() > p.len() && t.as_bytes()[..p.len()].eq_ignore_ascii_case(p.as_bytes());
    if t.eq_ignore_ascii_case("auto") || low("fit-content(") {
        return Some(GridTrack::Auto);
    }
    if t.eq_ignore_ascii_case("min-content") {
        return Some(GridTrack::MinContent);
    }
    if t.eq_ignore_ascii_case("max-content") {
        return Some(GridTrack::MaxContent);
    }
    if low("minmax(") && t.ends_with(')') {
        let inner = t.get(7..t.len() - 1)?;
        let comma = inner.rfind(',')?;
        return one_track(inner.get(comma + 1..)?, em);
    }
    if let Some(num) = strip_unit(t, "fr") {
        return hundredths(num).map(GridTrack::Fr);
    }
    if let Some(num) = t.strip_suffix('%') {
        return hundredths(num).map(GridTrack::Pct);
    }
    parse_px(t, em).map(|px| GridTrack::Px(px.min(u16::MAX as u32) as u16))
}

/* A number from 0 to 100 as hundredths, rounded. */
fn hundredths(num: &str) -> Option<u16> {
    let f = num.trim().parse::<f32>().ok()?;
    (f.is_finite() && (0.0..=100.0).contains(&f)).then_some((f * 100.0 + 0.5) as u16)
}
