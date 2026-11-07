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

//! A delivery frame as the router now sends it, 48 bytes with the window it
//! was routed to: the application reads the same event from it as from the
//! 40-byte frame, and parse_routed reads the window too.

use super::super::super::dispatch::parse_delivery;
use super::frames::delivery;
use nonos_app_skeleton::input::{parse_routed, InputKind};

fn with_window(mut f: Vec<u8>, window: Option<u32>) -> Vec<u8> {
    f.truncate(40);
    f[6..8].copy_from_slice(&u16::from(window.is_some()).to_le_bytes());
    f.extend_from_slice(&window.unwrap_or(0).to_le_bytes());
    f.extend_from_slice(&[0; 4]);
    f
}

#[test]
fn the_window_rides_beside_an_unchanged_event() {
    let f = with_window(delivery(5, 1, 12, 34), Some(9));
    assert_eq!(f.len(), 48);
    let (e, window) = parse_routed(&f).expect("a delivery");
    assert!(matches!(e.kind, InputKind::ButtonDown));
    assert_eq!((e.x, e.y, window), (12, 34, Some(9)));
    let old = parse_delivery(&f).expect("the application's own reading");
    assert_eq!((old.x, old.y, old.code), (e.x, e.y, e.code));
}

#[test]
fn a_frame_without_the_window_flag_names_none() {
    let f = with_window(delivery(3, 0, 5, 6), None);
    assert_eq!(parse_routed(&f).map(|(_, w)| w), Some(None));
    let short = delivery(3, 0, 5, 6);
    assert_eq!(parse_routed(&short[..40]).map(|(_, w)| w), Some(None));
}

#[test]
fn what_is_not_a_delivery_is_not_read() {
    let mut f = with_window(delivery(5, 1, 0, 0), Some(1));
    f[0] ^= 0xFF;
    assert!(parse_routed(&f).is_none());
    assert!(parse_routed(&f[..39]).is_none());
}
