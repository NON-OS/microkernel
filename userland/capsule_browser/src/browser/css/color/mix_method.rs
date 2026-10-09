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

use super::mix_lerp::HueWay;
use super::mix_space::Space;

/// The interpolation method: `in <space> [<way> hue]`.
pub(super) fn method(head: &str) -> Option<(Space, HueWay)> {
    let mut w = head.split_ascii_whitespace();
    if !w.next()?.eq_ignore_ascii_case("in") {
        return None;
    }
    let space = Space::named(&w.next()?.to_ascii_lowercase())?;
    let way = match (w.next(), w.next()) {
        (None, _) => HueWay::Shorter,
        (Some(k), Some(h)) if h.eq_ignore_ascii_case("hue") && space.hue().is_some() => {
            match k.to_ascii_lowercase().as_str() {
                "shorter" => HueWay::Shorter,
                "longer" => HueWay::Longer,
                "increasing" => HueWay::Increasing,
                "decreasing" => HueWay::Decreasing,
                _ => return None,
            }
        }
        _ => return None,
    };
    w.next().is_none().then_some((space, way))
}
