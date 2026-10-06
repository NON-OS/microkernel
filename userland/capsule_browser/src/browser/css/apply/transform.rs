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

use alloc::vec::Vec;

use crate::browser::css::calc::split_top::{items, words};
use crate::browser::layout::boxmodel::Affine;

use super::transform_fn::{angle, number, rel_len};
use super::trig::{cos, sin, tan};

/// A transform list folded into one 2D affine map; Some(None) is `none`,
/// None an unreadable value. 3D functions project without perspective:
/// rotateX squashes y, rotateY squashes x, translateZ leaves the plane.
pub(super) fn parse_transform(value: &str, fs: u32) -> Option<Option<Affine>> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("none") {
        return Some(None);
    }
    let mut m = Affine::IDENTITY;
    for word in words(v) {
        let open = word.find('(')?;
        let inner = word.get(open + 1..word.len() - 1).filter(|_| word.ends_with(')'))?;
        let args: Vec<&str> = items(inner).collect();
        let name = word[..open].to_ascii_lowercase();
        m = m.then(one(&name, &args, fs)?);
    }
    Some(Some(m))
}

fn one(name: &str, a: &[&str], fs: u32) -> Option<Affine> {
    let t = Affine::translate;
    let len = |i: usize| a.get(i).map_or(Some((0, 0)), |s| rel_len(s, fs));
    let num = |i: usize| number(a.get(i)?);
    let rot = |r: f32| Affine::linear(cos(r), sin(r), -sin(r), cos(r));
    let ang = |i: usize| a.get(i).map_or(Some(0.0), |s| angle(s));
    let sk = |i: usize| ang(i).map(tan);
    Some(match (name, a.len()) {
        ("translate", 1 | 2) | ("translate3d", 3) => t(len(0)?, len(1)?),
        ("translatex", 1) => t(len(0)?, (0, 0)),
        ("translatey", 1) => t((0, 0), len(0)?),
        ("translatez", 1) | ("perspective", 1) => Affine::IDENTITY,
        ("scale", 1) => Affine::linear(num(0)?, 0.0, 0.0, num(0)?),
        ("scale", 2) | ("scale3d", 3) => Affine::linear(num(0)?, 0.0, 0.0, num(1)?),
        ("scalex", 1) => Affine::linear(num(0)?, 0.0, 0.0, 1.0),
        ("scaley", 1) => Affine::linear(1.0, 0.0, 0.0, num(0)?),
        ("rotate", 1) | ("rotatez", 1) => rot(ang(0)?),
        ("rotatex", 1) => Affine::linear(1.0, 0.0, 0.0, cos(ang(0)?)),
        ("rotatey", 1) => Affine::linear(cos(ang(0)?), 0.0, 0.0, 1.0),
        ("skew", 1 | 2) => Affine::linear(1.0, sk(1)?, sk(0)?, 1.0),
        ("skewx", 1) => Affine::linear(1.0, 0.0, sk(0)?, 1.0),
        ("skewy", 1) => Affine::linear(1.0, sk(0)?, 0.0, 1.0),
        ("matrix", 6) => {
            let mut m = Affine::linear(num(0)?, num(1)?, num(2)?, num(3)?);
            (m.e[0], m.f[0]) = (num(4)?, num(5)?);
            m
        }
        _ => return None,
    })
}
