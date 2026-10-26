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
//! The faces the toolkit embeds are the brand's (nonos.software/brand):
//! Geist for text, Geist Medium for its bold, JetBrains Mono for data, and
//! Noto Sans for what Geist lacks. Read from the files face.rs includes, by
//! each file's own name table, with its licence beside it; and a character
//! Geist has no glyph for is drawn from the fallback, not as a box.

use nonos_toolkit::font::ttf::measure;
use std::path::Path;

const TOOLKIT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../toolkit");

/// A font file's family name: name ID 1, in the Windows or Unicode platform.
fn family(d: &[u8]) -> String {
    let be16 = |o: usize| u16::from_be_bytes([d[o], d[o + 1]]) as usize;
    let be32 = |o: usize| u32::from_be_bytes([d[o], d[o + 1], d[o + 2], d[o + 3]]) as usize;
    let at = (0..be16(4)).map(|i| 12 + 16 * i).find(|&r| &d[r..r + 4] == b"name").expect("name");
    let base = be32(at + 8);
    let strings = base + be16(base + 4);
    for r in (0..be16(base + 2)).map(|i| base + 6 + 12 * i) {
        if be16(r + 6) == 1 && matches!(be16(r), 0 | 3) {
            let s = &d[strings + be16(r + 10)..][..be16(r + 8)];
            return char::decode_utf16(s.chunks(2).map(|c| u16::from_be_bytes([c[0], c[1]])))
                .map(|c| c.unwrap_or('?'))
                .collect();
        }
    }
    String::new()
}

#[test]
fn the_toolkit_embeds_the_brand_faces() {
    let src = std::fs::read_to_string(format!("{TOOLKIT}/src/font/ttf/face.rs")).unwrap();
    for (name, file, fam, licence) in [
        ("BODY", "Geist-Regular.ttf", "Geist", "Geist-OFL.txt"),
        ("BOLD", "Geist-Medium.ttf", "Geist Medium", "Geist-OFL.txt"),
        ("MONO", "JetBrainsMono-Regular.ttf", "JetBrains Mono", "JetBrainsMono-OFL.txt"),
        ("WIDE", "NotoSans-Regular.ttf", "Noto Sans", "NotoSans-OFL.txt"),
    ] {
        let line = src.lines().find(|l| l.starts_with(&format!("const {name}:"))).unwrap();
        assert!(line.contains(&format!("assets/fonts/{file}\"")), "{name}: {line}");
        let fonts = Path::new(TOOLKIT).join("assets/fonts");
        assert_eq!(family(&std::fs::read(fonts.join(file)).unwrap()), fam, "{file}");
        assert!(fonts.join(licence).exists(), "{file} has its licence beside it");
    }
}

#[test]
fn a_character_geist_lacks_is_drawn_from_the_fallback() {
    // Ten of each at 200 px, so a few hundredths of an em between the box
    // and the glyph that stands in for it show in whole pixels.
    let run = |c: char| measure(&c.to_string().repeat(10), 200.0, false);
    let boxed = run('\u{378}');
    assert_ne!(run('\u{186}'), boxed, "the wider Latin from Noto Sans alone");
    assert_ne!(run('\u{2713}'), boxed, "a check mark from JetBrains Mono");
}
