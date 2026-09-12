// NONOS Operating System (AGPL-3.0-or-later)
//! What the send field must show while an amount is being typed.
//!
//! This is the number a reader checks before signing, so it has to echo the
//! keystrokes rather than a rounded version of them. The old field fixed four
//! decimal places, which printed a real 0.00005 as "0.0000" while the wallet
//! held something else entirely.

use crate::wallet::num::Amount;
use crate::wallet::paint::format_typed::format_typed;

const ETH: u32 = 18;

fn shown(keys: &str) -> String {
    let mut a = Amount::new();
    for c in keys.chars() {
        match c {
            '.' => a.start_point(),
            '<' => a.backspace(),
            d => a.digit(d as u8 - b'0', ETH),
        }
    }
    let mut out = [0u8; 64];
    let n = format_typed(&a, &mut out);
    String::from_utf8(out[..n].to_vec()).unwrap()
}

#[test]
fn an_untouched_field_reads_as_zero() {
    assert_eq!(shown(""), "0");
}

#[test]
fn whole_amounts_print_without_a_point() {
    assert_eq!(shown("1"), "1");
    assert_eq!(shown("250"), "250");
}

/// The point appears as soon as it is pressed, so the caret sits where the next
/// digit will land instead of jumping there later.
#[test]
fn the_point_shows_before_any_fraction_is_typed() {
    assert_eq!(shown("1."), "1.");
}

#[test]
fn a_leading_zero_fraction_keeps_its_place() {
    assert_eq!(shown("0.05"), "0.05", "a lost pad turns five hundredths into five tenths");
    assert_eq!(shown("0.005"), "0.005");
}

#[test]
fn the_figure_the_old_field_could_not_show() {
    assert_eq!(shown("0.00005"), "0.00005");
}

#[test]
fn a_bare_point_starts_at_zero() {
    assert_eq!(shown(".5"), "0.5");
}

#[test]
fn trailing_zeros_in_a_fraction_are_kept_as_typed() {
    assert_eq!(shown("1.500"), "1.500");
}

#[test]
fn backspace_is_reflected_immediately() {
    assert_eq!(shown("1.25<"), "1.2");
    assert_eq!(shown("1.25<<"), "1.");
    assert_eq!(shown("1.25<<<"), "1");
}

#[test]
fn a_figure_past_sixty_four_bits_prints_whole() {
    let mut a = Amount::new();
    for _ in 0..25 {
        a.digit(9, ETH);
    }
    let mut out = [0u8; 64];
    let n = format_typed(&a, &mut out);
    assert_eq!(String::from_utf8(out[..n].to_vec()).unwrap(), "9".repeat(25));
}
