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

use crate::browser::layout::filter_table::Tint;

/* A 3x4 color map plus alpha scale in floats while a list composes. */
type M = [f32; 13];

const ID: M = [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1.];

/// Compose a filter list's color functions, left to right, into one map.
/// blur(), drop-shadow() and url() are not color maps and are passed over;
/// None when nothing is left. Steps are not clamped between functions.
pub(super) fn parse_filter(v: &str) -> Option<Tint> {
    let (mut m, mut any, mut rest) = (ID, false, v.trim());
    while let Some(open) = rest.find('(') {
        let close = open + rest[open..].find(')')?;
        let (name, arg) = (rest[..open].trim(), rest[open + 1..close].trim());
        rest = &rest[close + 1..];
        if let Some(f) = step(&name.to_ascii_lowercase(), arg) {
            m = then(&m, &f);
            any = true;
        }
    }
    any.then(|| m.map(|x| (x * 65536.0) as i32))
}

/* One function's map; an amount may be a number or a percentage. */
fn step(name: &str, arg: &str) -> Option<M> {
    let n = |d: f32| match arg.strip_suffix('%') {
        Some(p) => p.trim().parse::<f32>().ok().map(|p| p / 100.0),
        None if arg.is_empty() => Some(d),
        None => arg.parse::<f32>().ok(),
    };
    let lin = |s: f32, o: f32| [s, 0., 0., o, 0., s, 0., o, 0., 0., s, o, 1.];
    Some(match name {
        "brightness" => lin(n(1.)?, 0.),
        "contrast" => lin(n(1.)?, 0.5 - 0.5 * n(1.)?),
        "invert" => lin(1. - 2. * n(1.)?.min(1.), n(1.)?.min(1.)),
        "opacity" => [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., n(1.)?.clamp(0., 1.)],
        "grayscale" => super::filter_mats::grayscale(n(1.)?.clamp(0., 1.)),
        "sepia" => super::filter_mats::sepia(n(1.)?.clamp(0., 1.)),
        "saturate" => super::filter_mats::saturate(n(1.)?.max(0.)),
        "hue-rotate" => super::filter_hue::hue(super::filter_hue::angle(arg)?),
        _ => return None,
    })
}

/* `a` then `b`: b's map applied to a's output. */
fn then(a: &M, b: &M) -> M {
    let mut out = [0.; 13];
    for r in 0..3 {
        for c in 0..4 {
            let s = (0..3).map(|k| b[r * 4 + k] * a[k * 4 + c]).sum::<f32>();
            out[r * 4 + c] = s + if c == 3 { b[r * 4 + 3] } else { 0. };
        }
    }
    out[12] = a[12] * b[12];
    out
}
