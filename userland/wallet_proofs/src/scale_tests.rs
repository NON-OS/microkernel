// NONOS Operating System (AGPL-3.0-or-later)
//! The type ramp must stay above the font's readable floor.
//!
//! This is the test that would have caught the flattest bug in the wallet. The
//! ramp was written to end twenty-five ad-hoc sizes, and then three of its own
//! six steps sat below seventeen pixels, where the toolkit clamps everything to
//! exactly seventeen. `LABEL` at 12.1, `SMALL` at 13.8 and `BODY` at 14.9 all
//! rendered identically, so a hundred and twenty-five pieces of text asked for
//! twenty-five sizes and got thirteen, and every screen came out flat.
//!
//! Nothing on screen said so and nothing in the build complained, because a size
//! the font refuses is not an error, it is a silently different number. So the
//! rule is asserted here instead: a step below the floor is a step that does not
//! exist, and the ramp is the only place sizes are written down.

use crate::wallet::paint::scale;
use crate::wallet::paint::scale::FLOOR;

const STEPS: [(&str, f32); 7] = [
    ("LABEL", scale::LABEL),
    ("SMALL", scale::SMALL),
    ("BODY", scale::BODY),
    ("VALUE", scale::VALUE),
    ("TITLE", scale::TITLE),
    ("HERO", scale::HERO),
    ("SPLASH", scale::SPLASH),
];

/// The one that matters: a step under the floor is silently rewritten by the
/// toolkit, so it is not a design decision, it is a lie in the source.
#[test]
fn no_step_is_below_the_readable_floor() {
    for (name, px) in STEPS {
        assert!(px >= FLOOR, "{name} is {px}px, under the {FLOOR}px floor: it renders as {FLOOR}");
    }
}

/// The ramp must never go backwards, or a "bigger" step would set smaller text.
#[test]
fn the_ramp_only_climbs() {
    for pair in STEPS.windows(2) {
        let (a, av) = pair[0];
        let (b, bv) = pair[1];
        assert!(bv >= av, "{b} ({bv}) is smaller than {a} ({av})");
    }
}

/// The upper steps are where size can carry hierarchy, so they must actually
/// differ. Two steps a point apart read as a mistake rather than as a level.
#[test]
fn the_upper_steps_are_far_enough_apart_to_read_as_different() {
    let upper = [
        ("BODY", scale::BODY),
        ("VALUE", scale::VALUE),
        ("TITLE", scale::TITLE),
        ("HERO", scale::HERO),
    ];
    for pair in upper.windows(2) {
        let (a, av) = pair[0];
        let (b, bv) = pair[1];
        assert!(bv - av >= 3.0, "{a} {av} and {b} {bv} are too close to tell apart");
    }
}

/// The bottom three coincide at the floor on purpose, because a caption cannot
/// be smaller than body text on this display. This is asserted so that the
/// coincidence stays deliberate: anyone who separates them must come here and
/// decide, rather than discovering the font has ignored them.
#[test]
fn the_bottom_three_sit_together_at_the_floor_by_design() {
    assert_eq!(scale::LABEL, FLOOR);
    assert_eq!(scale::SMALL, FLOOR);
    assert_eq!(scale::BODY, FLOOR);
}
