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

/// A font-weight descriptor (CSS Fonts 4 4.4): one weight or a range of
/// two in 1..=1000, normal and bold as 400 and 700, a reversed range
/// swapped. None for anything else, which leaves the face at normal.
pub(super) fn weight_range(v: &str) -> Option<(u16, u16)> {
    let one = |w: &str| match w.to_ascii_lowercase().as_str() {
        "normal" => Some(400.0),
        "bold" => Some(700.0),
        n => n.parse::<f32>().ok().filter(|x| (1.0..=1000.0).contains(x)),
    };
    let mut words = v.split_ascii_whitespace();
    let a = one(words.next()?)?;
    let b = words.next().map_or(Some(a), one)?;
    if words.next().is_some() {
        return None;
    }
    Some(((a.min(b) + 0.5) as u16, (a.max(b) + 0.5) as u16))
}

/// Whether a unicode-range list holds U+0061, the Latin small a; a face
/// without the descriptor covers everything.
pub(super) fn covers_latin(list: Option<&str>) -> bool {
    let Some(list) = list else { return true };
    list.split(',').any(|r| {
        let r = r.trim().trim_start_matches(['U', 'u']).trim_start_matches('+');
        let (lo, hi) = r.split_once('-').unwrap_or((r, r));
        let hex = |s: &str, fill: &str| u32::from_str_radix(&s.replace('?', fill), 16).ok();
        matches!((hex(lo, "0"), hex(hi, "F")), (Some(a), Some(b)) if a <= 0x61 && 0x61 <= b)
    })
}
