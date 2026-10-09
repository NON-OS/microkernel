// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Keyboard focus leaves the address bar: the bar used to start focused and
//! nothing but Enter ever cleared it, so Space typed a space into the URL
//! instead of scrolling and a page field never received a key.

use crate::browser::omnibox::{focus_after, route, Focus, Region, Route};
use nonos_app_skeleton::{KEY_PAGE_DOWN, KEY_UP};

#[test]
fn a_click_decides_who_has_the_keyboard() {
    assert_eq!(focus_after(Region::Omnibox, Some(4), Focus::Page), (Focus::Omnibox, None));
    assert_eq!(focus_after(Region::Page, Some(7), Focus::Omnibox), (Focus::Page, Some(7)));
    assert_eq!(focus_after(Region::Page, None, Focus::Omnibox), (Focus::Page, None));
    assert_eq!(focus_after(Region::Toolbar, Some(2), Focus::Omnibox), (Focus::Page, Some(2)));
    assert_eq!(focus_after(Region::Frame, None, Focus::Omnibox), (Focus::Omnibox, None));
}

#[test]
fn keys_reach_the_page_once_it_has_focus() {
    assert_eq!(route(Focus::Page, None, KEY_PAGE_DOWN), Route::Scroll);
    assert_eq!(route(Focus::Page, None, 0x20), Route::Scroll);
    assert_eq!(route(Focus::Page, None, b'x' as u32), Route::Page);
    assert_eq!(route(Focus::Page, Some(3), 0x20), Route::Field(3));
    assert_eq!(route(Focus::Omnibox, None, KEY_UP), Route::Omnibox);
    assert_eq!(route(Focus::Omnibox, None, 0x20), Route::Omnibox);
}
