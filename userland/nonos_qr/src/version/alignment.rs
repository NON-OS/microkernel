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

//! Where the alignment patterns' centres sit on each axis, versions 1 to 40,
//! by the rule ISO/IEC 18004 Annex E tabulates: the first at 6, the last at
//! size - 7, the rest an even step apart counted back from the last (26 for
//! version 32, the one exception to the formula).

/* Up to seven positions; the second value is how many there are. */
pub(crate) fn alignment_positions(version: u8) -> ([u8; 7], usize) {
    let mut out = [0u8; 7];
    if version == 1 {
        return (out, 0);
    }
    let v = version as u32;
    let count = (v / 7 + 2) as usize;
    let step =
        if v == 32 { 26 } else { (v * 4 + count as u32 * 2 + 1) / (count as u32 * 2 - 2) * 2 };
    let last = 17 + 4 * v - 7;
    out[0] = 6;
    for i in 1..count {
        out[count - i] = (last - (i as u32 - 1) * step) as u8;
    }
    (out, count)
}

#[cfg(test)]
mod tests {
    use super::alignment_positions;

    fn at(v: u8) -> alloc::vec::Vec<u8> {
        let (p, n) = alignment_positions(v);
        p[..n].to_vec()
    }

    #[test]
    fn versions_1_to_10_are_the_table_they_replace() {
        let want: [&[u8]; 10] = [
            &[],
            &[6, 18],
            &[6, 22],
            &[6, 26],
            &[6, 30],
            &[6, 34],
            &[6, 22, 38],
            &[6, 24, 42],
            &[6, 26, 46],
            &[6, 28, 50],
        ];
        for (i, w) in want.iter().enumerate() {
            assert_eq!(at(i as u8 + 1), *w, "version {}", i + 1);
        }
    }

    #[test]
    fn annex_e_rows_past_10() {
        // ISO/IEC 18004 Annex E, Table E.1.
        assert_eq!(at(14), [6, 26, 46, 66]);
        assert_eq!(at(21), [6, 28, 50, 72, 94]);
        assert_eq!(at(32), [6, 34, 60, 86, 112, 138]);
        assert_eq!(at(36), [6, 24, 50, 76, 102, 128, 154]);
        assert_eq!(at(40), [6, 30, 58, 86, 114, 142, 170]);
    }
}
