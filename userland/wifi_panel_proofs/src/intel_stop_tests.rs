// NONOS Operating System (AGPL-3.0-or-later)
//! The line a stopped Intel card puts on screen: the bring-up step it
//! stopped at (the driver's `outcome::STEP_*`, 1 to 12) in words and its
//! detail word in hex, read from the status reply after the stage byte;
//! nothing while the card is ready or restarting, and every line fits the
//! panels whole.

use crate::intel_step::intel_step_text;
use crate::stop::StopWhy;

fn text(why: StopWhy) -> Option<String> {
    let mut out = [0u8; 64];
    why.text(&mut out).map(|t| String::from_utf8(t.to_vec()).unwrap())
}

// The status reply past its header: stage, step, then the detail word.
fn status(stage: u8, step: u8, detail: u32) -> Vec<u8> {
    let mut s = vec![stage, step];
    s.extend_from_slice(&detail.to_le_bytes());
    s.extend_from_slice(&[0; 24]);
    s
}

#[test]
fn every_driver_step_has_words_and_no_other_does() {
    assert!((1u8..=12).all(|s| intel_step_text(s).is_some()));
    assert_eq!((intel_step_text(0), intel_step_text(13)), (None, None));
    assert_eq!(intel_step_text(7), Some("firmware never came alive"));
}

#[test]
fn a_stopped_card_names_its_step_and_detail() {
    let why = StopWhy::intel(&status(4, 8, 0x0101_C801)).unwrap();
    assert_eq!(why, StopWhy::Intel(8, 0x0101_C801));
    let line = text(why).unwrap();
    assert_eq!(line, "stopped at 8: a firmware setup command failed (0x0101c801)");
    let alive = text(StopWhy::intel(&status(4, 7, 0x30)).unwrap()).unwrap();
    assert_eq!(alive, "stopped at 7: firmware never came alive (0x00000030)");
    assert_eq!(text(StopWhy::Intel(13, 0)), None, "a step this build does not know");
}

#[test]
fn a_ready_or_restarting_card_or_a_short_reply_says_nothing() {
    assert_eq!(StopWhy::intel(&status(0, 0, 0)), None, "ready");
    assert_eq!(StopWhy::intel(&status(9, 2, 0)), None, "restarting: the step is its try");
    assert_eq!(StopWhy::intel(&status(4, 0, 0x30)), None, "no step named");
    assert_eq!(StopWhy::intel(&status(4, 7, 0x30)[..5]), None, "no whole detail word");
}

#[test]
fn every_line_fits_the_panels_whole() {
    for step in 1u8..=12 {
        let t = text(StopWhy::Intel(step, u32::MAX)).unwrap();
        assert!(t.ends_with("(0xffffffff)"), "not cut at 64 bytes: {t}");
        assert!("Intel Wi-Fi: ".len() + t.len() <= 72, "the Settings row: {t}");
    }
}
