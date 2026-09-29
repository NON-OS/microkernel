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

/* Whether an @font-face unicode-range covers Latin text, tested at 'A'. The
 * registry keeps one face per family and weight, and a site that splits its
 * fonts into subsets (latin, latin-ext, vietnamese, cyrillic) must not end up
 * with a subset that has no Latin letters. No range means the whole face. */
pub(super) fn covers_latin(range: Option<&str>) -> bool {
    let Some(range) = range else {
        return true;
    };
    range.split(',').any(|item| {
        let item = item.trim();
        let hex = item.strip_prefix("U+").or_else(|| item.strip_prefix("u+")).unwrap_or(item);
        let (lo, hi) = match hex.split_once('-') {
            Some((lo, hi)) => (lo.replace('?', "0"), hi.replace('?', "F")),
            None => (hex.replace('?', "0"), hex.replace('?', "F")),
        };
        let lo = u32::from_str_radix(lo.trim(), 16);
        let hi = u32::from_str_radix(hi.trim(), 16);
        matches!((lo, hi), (Ok(lo), Ok(hi)) if lo <= 0x41 && 0x41 <= hi)
    })
}
