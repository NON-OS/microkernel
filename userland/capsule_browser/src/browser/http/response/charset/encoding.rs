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

use alloc::string::String;

use super::{big5, euc_jp, euc_kr, gb18030, iso2022jp, shift_jis, single_byte, utf16};

/* The encodings of the Encoding Standard. GBK decodes as gb18030, as the
standard defines it; `Single(n)` is the n-th single-byte table. */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    Utf8,
    Utf16Be,
    Utf16Le,
    Single(u8),
    Gb18030,
    Big5,
    EucJp,
    Iso2022Jp,
    ShiftJis,
    EucKr,
    Replacement,
    XUserDefined,
}

/* windows-1252's place in single_byte.idx, which labels.txt gives as
`#20`; the default for bytes that declare nothing and are not UTF-8. */
pub const WINDOWS_1252: Encoding = Encoding::Single(20);

impl Encoding {
    /* Appends the text of `b` (its BOM already removed) to `out`, each
    error as U+FFFD, following the standard's decoder for `self`. */
    pub fn decode(self, b: &[u8], out: &mut String) {
        match self {
            Encoding::Utf8 => out.push_str(&String::from_utf8_lossy(b)),
            Encoding::Utf16Be => utf16::decode(b, true, out),
            Encoding::Utf16Le => utf16::decode(b, false, out),
            Encoding::Single(n) => single_byte::decode(usize::from(n), b, out),
            Encoding::Gb18030 => gb18030::decode(b, out),
            Encoding::Big5 => big5::decode(b, out),
            Encoding::EucJp => euc_jp::decode(b, out),
            Encoding::Iso2022Jp => iso2022jp::decode(b, out),
            Encoding::ShiftJis => shift_jis::decode(b, out),
            Encoding::EucKr => euc_kr::decode(b, out),
            Encoding::Replacement if !b.is_empty() => out.push('\u{FFFD}'),
            Encoding::Replacement => {}
            Encoding::XUserDefined => single_byte::user_defined(b, out),
        }
    }

    /* A <meta> or @charset naming UTF-16 means UTF-8: the bytes being read
    are ASCII-compatible, which UTF-16 is not. */
    pub fn ascii_compatible(self) -> Encoding {
        match self {
            Encoding::Utf16Be | Encoding::Utf16Le => Encoding::Utf8,
            e => e,
        }
    }
}
