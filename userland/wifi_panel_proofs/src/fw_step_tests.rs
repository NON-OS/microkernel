// NONOS Operating System (AGPL-3.0-or-later)
//! The panel names every firmware step the RTL8821CE driver reports (codes 1 to
//! 11 of its `FwStep`), says nothing for no failure, and nothing for a code a
//! newer driver adds, so the stage line stands alone then.

use crate::fw_step::firmware_step_text;

#[test]
fn every_driver_step_has_a_line() {
    for code in 1u8..=11 {
        assert!(firmware_step_text(code).is_some(), "code {code}");
    }
}

#[test]
fn no_failure_and_unknown_codes_have_none() {
    assert_eq!(firmware_step_text(0), None);
    assert_eq!(firmware_step_text(12), None);
    assert_eq!(firmware_step_text(255), None);
}

#[test]
fn every_line_fits_the_panel_with_its_prefix_and_register() {
    // "Firmware stopped: " + text + " (0x" + 4 hex + ")" within the 72-byte row.
    for code in 1u8..=11 {
        let t = firmware_step_text(code).unwrap();
        assert!(18 + t.len() + 4 + 4 < 72, "code {code}: {t}");
    }
}

#[test]
fn the_staging_step_says_the_card_never_fetched() {
    assert_eq!(firmware_step_text(5), Some("the card never fetched the staged chunk"));
}
