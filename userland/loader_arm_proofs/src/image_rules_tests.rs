// NONOS Operating System (AGPL-3.0-or-later)
//! Which images are loaded which way, and how much each is given.

use crate::image_rules::{footprint_rule, route, Route};

#[test]
fn x86_64_keeps_its_two_ways_and_refuses_the_rest() {
    for upper in [false, true] {
        assert_eq!(route(true, false, upper, false), Route::Exec);
        assert_eq!(route(false, true, upper, false), Route::Dyn);
        assert!(matches!(route(false, false, upper, false), Route::Refuse(_)));
    }
}

#[test]
fn a_relocating_cpu_takes_only_a_low_et_dyn() {
    assert_eq!(route(false, true, false, true), Route::Block);
    assert!(matches!(route(false, true, true, true), Route::Refuse(_)), "linked high");
    assert!(matches!(route(true, false, false, true), Route::Refuse(_)), "an ET_EXEC");
    assert!(matches!(route(true, false, true, true), Route::Refuse(_)), "an ET_EXEC mapped high");
    assert!(matches!(route(false, false, false, true), Route::Refuse(_)));
}

#[test]
fn the_header_raises_the_span_and_never_lowers_it() {
    assert_eq!(footprint_rule(Some(0x579_0000), 0x560_0000, true), Some(0x579_0000));
    assert_eq!(footprint_rule(Some(0x1000), 0x560_0000, true), Some(0x560_0000));
    assert_eq!(footprint_rule(None, 0x560_0000, false), Some(0x560_0000), "x86_64 needs none");
}

#[test]
fn a_required_header_that_is_missing_refuses() {
    assert_eq!(footprint_rule(None, 0x560_0000, true), None);
}
