// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Which stylesheets apply: <link> media, rel=alternate, disabled and type,
//! <style media>, and the inline text budget. A print or dark-theme sheet
//! that is applied restyles the whole page, so each rule is pinned here.

use crate::browser::layout::boxmodel::Content;
use crate::browser::sheet::sheet_applies;
use crate::probe::Page;

fn applies(attrs: &[(&str, &str)]) -> bool {
    sheet_applies(|k| attrs.iter().find(|(n, _)| *n == k).map(|(_, v)| *v), (1336, 680))
}

pub(super) fn color(p: &Page, word: &str) -> u32 {
    match &p.word(word).content {
        Content::Text { color, .. } => *color,
        _ => unreachable!("word() only returns text fragments"),
    }
}

#[test]
fn only_matching_enabled_stylesheets_apply() {
    assert!(applies(&[("rel", "stylesheet")]));
    assert!(applies(&[("rel", "StyleSheet"), ("media", "all")]));
    assert!(applies(&[("rel", "stylesheet"), ("media", "screen and (min-width:600px)")]));
    assert!(applies(&[("rel", "stylesheet"), ("media", "print, screen")]));
    assert!(applies(&[("rel", "stylesheet"), ("type", "text/css; charset=utf-8")]));
    assert!(!applies(&[("rel", "stylesheet"), ("media", "print")]));
    assert!(!applies(&[("rel", "stylesheet"), ("media", "(prefers-color-scheme: dark)")]));
    assert!(!applies(&[("rel", "stylesheet"), ("media", "screen and (max-width:600px)")]));
    assert!(!applies(&[("rel", "alternate stylesheet")]));
    assert!(!applies(&[("rel", "stylesheet"), ("disabled", "")]));
    assert!(!applies(&[("rel", "stylesheet"), ("type", "text/less")]));
    assert!(!applies(&[("rel", "icon")]));
    assert!(!applies(&[("rel", "stylesheet"), ("media", "x{} a{")]));
}

#[test]
fn style_media_applies_only_where_it_matches() {
    let html = "<style media=print>#a{color:#f00}</style>\
                <style media='(prefers-color-scheme: dark)'>#a{color:#0f0}</style>\
                <style media='screen and (min-width: 600px)'>#b{color:#00f}</style>\
                <style type=text/less>#b{color:#f00}</style>\
                <p id=a>a</p><p id=b>b</p>";
    let p = Page::at(html, (1336, 680));
    assert_eq!(color(&p, "a"), 0xFF1A_1A1A, "print and dark sheets are skipped");
    assert_eq!(color(&p, "b"), 0xFF00_00FF, "a matching media sheet applies");
}
