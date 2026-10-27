// NONOS Operating System (AGPL-3.0-or-later)
//! The line a stopped RTL8821CE puts on screen: the refused claim step with
//! the broker's errno in words and in digits, or the firmware step; nothing
//! for a code this build does not know, and every line fits Settings' row.

use crate::stop::StopWhy;

fn text(why: StopWhy) -> Option<String> {
    let mut out = [0u8; 64];
    why.text(&mut out).map(|t| String::from_utf8(t.to_vec()).unwrap())
}

#[test]
fn a_refused_claim_names_its_step_and_errno() {
    let busy = text(StopWhy::Claim(1, 16)).unwrap();
    assert_eq!(busy, "not claimed: claim refused, busy (-16)");
    let map = text(StopWhy::Claim(3, 1)).unwrap();
    assert_eq!(map, "not claimed: register map refused, not permitted (-1)");
    let odd = text(StopWhy::Claim(2, 125)).unwrap();
    assert_eq!(odd, "not claimed: memory enable refused, error (-125)");
    assert!(text(StopWhy::Claim(2, 4000)).unwrap().ends_with("(-999)"));
}

#[test]
fn an_unknown_step_says_nothing() {
    assert_eq!(text(StopWhy::Claim(0, 16)), None);
    assert_eq!(text(StopWhy::Claim(4, 16)), None);
    assert_eq!(text(StopWhy::Firmware(0)), None);
    let fw = text(StopWhy::Firmware(5)).unwrap();
    assert_eq!(fw, "firmware stopped: the card never fetched the staged chunk");
}

#[test]
fn every_claim_line_fits_the_settings_row() {
    for step in 1..=3 {
        for errno in [1, 11, 12, 14, 16, 19, 22, 999] {
            let t = text(StopWhy::Claim(step, errno)).unwrap();
            assert!("Card ".len() + t.len() <= 72, "{t}");
        }
    }
}
