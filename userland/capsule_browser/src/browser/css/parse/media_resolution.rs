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

use nonos_toolkit::design::scale::scale;

/* resolution and -webkit-device-pixel-ratio, with their min- and max-
 * forms, against the display's device pixels per CSS px. None for any
 * other feature; a value that does not parse fails closed. */
pub(super) fn resolution(name: &str, value: Option<&str>) -> Option<bool> {
    let (bound, ratio) = match name.strip_prefix("-webkit-") {
        Some(n) => (n.strip_suffix("device-pixel-ratio")?, true),
        None => (name.strip_suffix("resolution")?, false),
    };
    let dppx = value.and_then(|v| if ratio { v.trim().parse().ok() } else { dppx(v) });
    let Some(v) = dppx else { return Some(false) };
    let z = scale().quarters() as f32 / 4.0;
    Some(match bound {
        "min-" => z >= v - 0.001,
        "max-" => z <= v + 0.001,
        "" => (z - v).abs() < 0.001,
        _ => false,
    })
}

fn dppx(v: &str) -> Option<f32> {
    let v = v.trim();
    let units = [("dppx", 1.0), ("dpcm", 96.0 / 2.54), ("dpi", 96.0), ("x", 1.0)];
    let (n, per) = units.iter().find_map(|(u, per)| Some((v.strip_suffix(u)?, *per)))?;
    n.trim().parse::<f32>().ok().map(|n| n / per)
}
