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

//! Which listings install from an image, and where a mirror serves them.

use crate::install::image::named::{hex, image, image_path, list_path, PREFIX};

#[test]
fn an_image_listing_names_its_package() {
    assert_eq!(PREFIX, "nonos-image-");
    assert_eq!(image("nonos-image-telegram-desktop"), Some("telegram-desktop"));
    assert_eq!(image("nonos-image-libstdc++"), Some("libstdc++"));
    for no in ["telegram-desktop", "nonos-rg", "nonos-image-", "nonos-image--x", "nonos-image-A",
               "nonos-image-a/b", "nonos-image-.x"] {
        assert_eq!(image(no), None, "{no}");
    }
    assert_eq!(image(&format!("nonos-image-{}", "a".repeat(65))), None);
}

#[test]
fn the_paths_are_the_pin_in_hex() {
    let mut pin = [0u8; 32];
    pin[0] = 0xab;
    pin[31] = 0x01;
    assert_eq!(hex(&pin), format!("ab{}01", "00".repeat(30)));
    assert_eq!(list_path(&pin), format!("/linux/{}.chunks", hex(&pin)));
    assert_eq!(image_path(&pin), format!("/linux/{}.image", hex(&pin)));
}

#[test]
fn a_bare_word_names_the_image_package_it_would_be() {
    use crate::install::image::named::of_word;
    assert_eq!(of_word(b"libreoffice").as_deref(), Some("nonos-image-libreoffice"));
    assert_eq!(of_word(b"telegram-desktop").as_deref(), Some("nonos-image-telegram-desktop"));
    assert_eq!(of_word(b"nonos-image-libreoffice"), None, "already an image's");
    for no in [&b"/usr/bin/libreoffice"[..], b"LibreOffice", b"", b"-x", b"a b", &[0xff]] {
        assert_eq!(of_word(no), None, "{no:?}");
    }
    assert!(of_word(&[b'a'; 65]).is_none(), "past the 64 bytes an image name holds");
    assert!(of_word(&[b'a'; 64]).is_some());
}
