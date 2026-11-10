// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The own pages (History, Bookmarks, Downloads, Tabs, Settings) carry shared
//! widgets that take the OS theme's roles, so their ground does too. The
//! ground takes no appearance, so a Light or Dark appearance that disagrees
//! with the OS scheme cannot move it; each scheme, high contrast included, is
//! checked against every named palette the appearances could put in force.

use nonos_toolkit::theme::select::theme_of;
use nonos_toolkit::theme::{from_theme, Roles, SCHEMES};

use crate::browser::pages::look::ground_in;
use crate::browser::tokens::system::from_roles;
use crate::browser::tokens::{DARK, LIGHT};

use super::contrast_tests::contrast;

const AA: f64 = 4.5;

fn every_scheme() -> Vec<(String, Roles)> {
    let mut all: Vec<(String, Roles)> = (0..SCHEMES.len())
        .map(|i| (format!("scheme {i}"), from_theme(&theme_of(i as u8, false))))
        .collect();
    all.push(("high contrast".to_string(), from_theme(&theme_of(0, true))));
    all
}

#[test]
fn own_page_text_reads_on_the_ground_under_every_appearance_and_scheme() {
    for (scheme, r) in every_scheme() {
        let named = [("light", LIGHT), ("dark", DARK), ("system", from_roles(&r))];
        for (a, p) in named {
            let g = ground_in(&r);
            assert!(contrast(p.ink, p.ground) >= AA, "{a} on {scheme}: the start page");
            let inks = [
                ("header title", r.text),
                ("header note", r.muted),
                ("row label", r.text),
                ("pill label", r.text),
            ];
            for (what, ink) in inks {
                let c = contrast(ink.argb(), g);
                assert!(c >= AA, "{a} on {scheme}: {what} on the page ground is {c:.2}:1");
            }
        }
    }
}
