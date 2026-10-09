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

//! The review names every answer the settings service refused, on lines
//! that fit the column on every canvas, and says nothing when all were taken.

use crate::displays::canvases;
use crate::unsaved::{said, ALL, KEYBOARD, PERSISTENCE, QWEN, THEN};
use crate::wizard_layout::{column, layout};
use crate::{measure, Face};

fn lines(mask: u16) -> (String, String) {
    let (mut a, mut b) = ([0u8; 96], [0u8; 96]);
    let (n, m) = said(mask, &mut a, &mut b);
    (String::from_utf8_lossy(&a[..n]).into(), String::from_utf8_lossy(&b[..m]).into())
}

#[test]
fn every_answer_taken_says_nothing() {
    assert_eq!(lines(0), (String::new(), String::new()));
}

#[test]
fn a_refused_qwen_tier_is_named() {
    let (a, b) = lines(QWEN);
    assert_eq!(a, "Not applied to this session: Qwen model");
    assert!(b.is_empty());
    let (a, _) = lines(KEYBOARD | QWEN);
    assert_eq!(a, "Not applied to this session: Keyboard, Qwen model");
}

#[test]
fn no_settings_service_is_said_as_such() {
    let (a, b) = lines(ALL);
    assert!(a.contains("no settings service"), "{a}");
    assert!(b.is_empty());
}

#[test]
fn every_name_is_said_and_none_twice() {
    for mask in 1..ALL {
        let (a, b) = lines(mask);
        let head = "Not applied to this session: ";
        assert!(a.starts_with(head), "{a}");
        let mut named: Vec<&str> = a[head.len()..].split(", ").collect();
        if !b.is_empty() {
            named.extend(b.split(", "));
        }
        let mut seen = named.clone();
        seen.dedup();
        assert_eq!(named.len(), mask.count_ones() as usize, "{mask:#x}: {a} / {b}");
        assert_eq!(seen.len(), named.len(), "{mask:#x}: a name twice");
        assert!(a.len() < 96 && b.len() < 96, "{mask:#x}: {a} / {b}");
    }
    let (a, b) = lines(ALL & !PERSISTENCE);
    assert!(b.ends_with("Computer"), "{a} / {b}");
}

#[test]
fn every_combination_fits_the_column() {
    for c in canvases() {
        let l = layout(c.width, c.height);
        let (_, col_w) = column(c.width, l.unit);
        let fits = |s: &str| {
            let w = measure(s, Face::Body, l.body_px, 0.0);
            assert!(w <= col_w, "{}x{}: \"{s}\" is {w} wide, has {col_w}", c.width, c.height);
        };
        fits(&String::from_utf8_lossy(THEN));
        for mask in 1..=ALL {
            let (a, b) = lines(mask);
            fits(&a);
            fits(&b);
        }
    }
}
