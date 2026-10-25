// NONOS Operating System (AGPL-3.0-or-later)
//! Which tab takes the command: the tile's untouched tab, its own, or an idle prompt.

use crate::handed_pick::{pick, Active, Pick};

const MAX: usize = 9;

#[test]
fn the_untouched_tab_a_tile_just_opened_takes_the_command() {
    let fresh = Active { fresh: true, idle: true };
    assert_eq!(pick(fresh, 1, MAX), Pick::Current);
    assert_eq!(pick(fresh, MAX, MAX), Pick::Current);
}

#[test]
fn a_used_tab_is_kept_and_the_command_gets_its_own() {
    for active in [
        Active { fresh: false, idle: true },
        Active { fresh: false, idle: false },
        // Something typed on the splash, or a program running from it.
        Active { fresh: true, idle: false },
    ] {
        assert_eq!(pick(active, 1, MAX), Pick::NewTab);
        assert_eq!(pick(active, MAX - 1, MAX), Pick::NewTab);
    }
}

#[test]
fn with_every_tab_taken_only_an_idle_prompt_will_do() {
    assert_eq!(pick(Active { fresh: false, idle: true }, MAX, MAX), Pick::Current);
    assert_eq!(pick(Active { fresh: false, idle: false }, MAX, MAX), Pick::Nowhere);
    assert_eq!(pick(Active { fresh: true, idle: false }, MAX, MAX), Pick::Nowhere);
}
