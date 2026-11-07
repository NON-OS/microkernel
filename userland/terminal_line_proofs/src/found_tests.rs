// NONOS Operating System (AGPL-3.0-or-later)
//! The search match's ground: under System in every scheme and high contrast,
//! and in every named profile, the text on it clears the body floor and the
//! match still shows against the ground.

use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::{roles, store, theme_of, SCHEMES};

use crate::term::theme::found::found;
use crate::term::theme::types::Theme;
use crate::term::theme::{profiles, system};

fn every_theme() -> Vec<(String, Theme)> {
    let mut out = Vec::new();
    let schemes = (0..SCHEMES.len() as u8).map(|i| (i, false)).chain([(0, true)]);
    for (i, hc) in schemes {
        store::replace(theme_of(i, hc));
        out.push((format!("system {i} hc {hc}"), system::theme(&roles())));
    }
    store::replace(theme_of(0, false));
    let named = (0..profiles::COUNT).map(|i| (format!("profile {i}"), *profiles::by_index(i)));
    out.extend(named);
    out
}

#[test]
fn text_on_a_match_clears_the_body_floor_in_every_theme() {
    let _g = crate::strip_boxes::serial();
    for (name, t) in every_theme() {
        assert!(at_least(t.fg, found(&t), BODY), "{name}: text on a match is under 4.5:1");
    }
}

#[test]
fn a_match_still_shows_against_the_ground() {
    let _g = crate::strip_boxes::serial();
    for (name, t) in every_theme() {
        assert_ne!(found(&t), t.bg | 0xFF00_0000, "{name}: a match is the ground");
    }
}
