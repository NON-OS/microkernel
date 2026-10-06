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

fn be(words: &[i32]) -> Vec<u8> {
    words.iter().flat_map(|w| (*w as i16).to_be_bytes()).collect()
}

/*
 * A TrueType face in miniature, the shape of a hostile web font: unitsPerEm
 * 16 and a single glyph, which every character maps to, that is an `n` x `n`
 * unit square standing n/2 below the baseline. At 16 px one unit is one
 * pixel, so each glyph box is `n` x `n` pixels.
 */
pub fn square_face(n: i32) -> Vec<u8> {
    let h = n / 2;
    let mut glyf = be(&[1, 0, -h, n, n - h, 3, 0]);
    glyf.extend([1u8; 4]);
    glyf.extend(be(&[0, 0, n, 0, -h, n, 0, -n]));
    let magic = [0x5f0f, 0x3cf5];
    let mut head = be(&[1, 0, 1, 0, 0, 0, magic[0], magic[1], 0, 16, 0, 0, 0, 0]);
    head.extend(be(&[0, 0, 0, 0, 0, -h, n, n - h, 0, 8, 2, 0, 0]));
    let hhea = be(&[1, 0, 12, -4, 0, 16, 0, 0, n, 1, 0, 0, 0, 0, 0, 0, 0, 1]);
    let tables: [(&[u8; 4], Vec<u8>); 6] = [
        (b"glyf", glyf.clone()),
        (b"head", head),
        (b"hhea", hhea),
        (b"hmtx", be(&[16, 0])),
        (b"loca", be(&[0, glyf.len() as i32 / 2])),
        (b"maxp", be(&[0, 0x5000, 1])),
    ];
    let mut out = be(&[1, 0, 6, 64, 2, 32]);
    let mut at = 12 + 16 * tables.len();
    for (tag, data) in &tables {
        out.extend(*tag);
        out.extend([0u8; 4]);
        out.extend((at as u32).to_be_bytes());
        out.extend((data.len() as u32).to_be_bytes());
        at += data.len().next_multiple_of(4);
    }
    for (_, data) in &tables {
        out.extend(data);
        out.resize(out.len().next_multiple_of(4), 0);
    }
    out
}
