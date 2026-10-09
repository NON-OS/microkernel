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

use super::encoding::Encoding;

/* Every encoding with its labels, one per line: the name (`#n` for the
n-th single-byte table), then each label, lowercase. */
static LABELS: &str = include_str!("index/labels.txt");

pub fn is_ascii_space(c: u8) -> bool {
    matches!(c, b'\t' | b'\n' | 0x0C | b'\r' | b' ')
}

/* "Get an encoding": the encoding `label` names once ASCII whitespace is
trimmed, compared without ASCII case, or None for no known label. */
pub fn encoding(label: &[u8]) -> Option<Encoding> {
    let start = label.iter().position(|&c| !is_ascii_space(c))?;
    let end = label.iter().rposition(|&c| !is_ascii_space(c))? + 1;
    let label = &label[start..end];
    for line in LABELS.lines() {
        let mut words = line.split(' ');
        let Some(name) = words.next() else { continue };
        if words.any(|w| w.as_bytes().eq_ignore_ascii_case(label)) {
            return named(name);
        }
    }
    None
}

fn named(name: &str) -> Option<Encoding> {
    if let Some(n) = name.strip_prefix('#') {
        return n.parse().ok().map(Encoding::Single);
    }
    Some(match name {
        "utf-8" => Encoding::Utf8,
        "gbk" | "gb18030" => Encoding::Gb18030,
        "big5" => Encoding::Big5,
        "euc-jp" => Encoding::EucJp,
        "iso-2022-jp" => Encoding::Iso2022Jp,
        "shift_jis" => Encoding::ShiftJis,
        "euc-kr" => Encoding::EucKr,
        "replacement" => Encoding::Replacement,
        "utf-16be" => Encoding::Utf16Be,
        "utf-16le" => Encoding::Utf16Le,
        "x-user-defined" => Encoding::XUserDefined,
        _ => return None,
    })
}
