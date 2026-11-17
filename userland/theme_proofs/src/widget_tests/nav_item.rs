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

//! NavItem: label, icon and the active look stay in the box, the hit test
//! takes the body it draws, and label, icon and count read in every state,
//! compact or not.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::NavItem;

use super::agree::assert_agree;
use super::bounds::assert_bounded;
use super::boxes::boxes;
use super::hide::Hide;
use super::legible::{assert_legible, Off, Read};
use super::samples::LONG;
use super::serial::serial;
use crate::wcag::{BODY, LARGE};

fn variants() -> Vec<(String, NavItem<'static>)> {
    let named = |n: &str, v| (n.to_string(), v);
    let icon = NavItem::new("Network").icon(IconId::SettingsNetwork);
    vec![
        named("label", NavItem::new("Grüße — 日本語")),
        named("icon", icon),
        named("active", NavItem::new("Network").active(true)),
        named("active with icon", icon.active(true)),
        named("long with icon", NavItem::new(LONG).icon(IconId::Files).active(true)),
        named("count", NavItem::new("Tabs").count(12)),
        named("count with icon", icon.count(u32::MAX).active(true)),
        named("long with count", NavItem::new(LONG).icon(IconId::Files).count(7)),
        named("compact", icon.compact()),
        named("compact active", icon.count(3).active(true).compact()),
        named("compact without icon", NavItem::new("Network").compact()),
    ]
}

#[test]
fn nav_item_paints_inside_its_box() {
    let _g = serial();
    assert_bounded(&variants());
}

#[test]
fn nav_item_hit_takes_the_body_it_draws() {
    let _g = serial();
    assert_agree(&variants(), boxes, |n, at, x, y| n.hit(at, x, y));
}

#[test]
fn nav_item_label_and_icon_read_active_or_not() {
    let _g = serial();
    let item = NavItem::new("Grüße — 日本語").icon(IconId::SettingsNetwork);
    let label = || Off::Hide(Hide::Text("Grüße — 日本語"));
    assert_legible(&[
        Read { what: "label", on: item, off: label(), floor: BODY },
        Read { what: "active label", on: item.active(true), off: label(), floor: BODY },
        Read { what: "icon", on: item, off: Off::Hide(Hide::Icons), floor: LARGE },
        Read {
            what: "active icon",
            on: item.active(true),
            off: Off::Hide(Hide::Icons),
            floor: LARGE,
        },
    ]);
}

#[test]
fn nav_item_count_and_compact_icon_read_active_or_not() {
    let _g = serial();
    let item = NavItem::new("Tabs").icon(IconId::SettingsNetwork).count(12);
    let (count, icons) = (|| Off::Hide(Hide::Text("12")), || Off::Hide(Hide::Icons));
    let on = item.active(true);
    assert_legible(&[
        Read { what: "count", on: item, off: count(), floor: BODY },
        Read { what: "active count", on, off: count(), floor: BODY },
        Read { what: "compact icon", on: item.compact(), off: icons(), floor: LARGE },
        Read { what: "active compact", on: on.compact(), off: icons(), floor: LARGE },
    ]);
}
