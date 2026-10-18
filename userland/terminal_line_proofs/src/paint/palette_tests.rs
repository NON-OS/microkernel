// NONOS Operating System (AGPL-3.0-or-later)
/*
 * The sixteen colours against every ground the terminal ships, measured as
 * WCAG contrast. Blue at xterm's #0000EE on #07090B is 2.2:1, which is why
 * a table header printed in it could not be read.
 */

use crate::ansi::base16_for;

/// Every theme background in term/theme/profiles.rs.
const GROUNDS: [u32; 4] = [0x07_090B, 0x0B_0E12, 0xF2_F4F7, 0x00_0308];

fn luminance(c: u32) -> f64 {
    let lin = |v: u32| {
        let s = v as f64 / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin((c >> 16) & 0xFF) + 0.7152 * lin((c >> 8) & 0xFF) + 0.0722 * lin(c & 0xFF)
}

fn contrast(a: u32, b: u32) -> f64 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/*
 * Index 0 is black and 8 grey: programs use them for rules and dim text,
 * so they only need to be visible. Every other colour carries text and
 * meets the 4.5:1 WCAG asks of body text.
 */
#[test]
fn every_text_colour_is_readable_on_every_ground() {
    for bg in GROUNDS {
        let set = base16_for(bg);
        for (i, &c) in set.iter().enumerate() {
            let need = if matches!(i, 0 | 8) { 1.3 } else { 4.5 };
            let r = contrast(c, bg);
            assert!(r >= need, "colour {i} {c:06x} on {bg:06x} is {r:.2}:1");
        }
    }
}

#[test]
fn the_old_blue_was_unreadable() {
    assert!(contrast(0x0000EE, 0x07_090B) < 3.0);
}
