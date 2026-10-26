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
//! A line too long for its width is cut with an ellipsis on a character
//! The chrome's say in the pointer's shape (capsule_desktop_shell state/ctx/
//! pointer_shape.rs): the arrow over its own pixels, withdrawn over its
//! transparent ones, told only when that changes; and the codes it tells by
//! are the compositor's.

#[path = "../../capsule_desktop_shell/src/state/ctx/pointer_shape.rs"]
mod pointer_shape;

use pointer_shape::PointerShape;

const CLIENT: &str =
    include_str!("../../capsule_desktop_shell/src/compositor_client/cursor_shape.rs");
const DECODE: &str = include_str!("../../compositor/src/requests/pointer.rs");
const SHAPES: &str = include_str!("../../compositor/src/frame_pacer/cursor_shapes.rs");

#[test]
fn pointer_shape_arrow_over_the_shells_pixels_withdrawn_elsewhere() {
    let fresh = PointerShape::default();
    assert_eq!(fresh.at, None, "no pointer seen yet");
    assert_eq!(fresh.change(0xFF10_1A22), Some(true), "a menu: the arrow");
    assert_eq!(fresh.change(0x0000_0000), Some(false), "nothing drawn: withdrawn");
    assert_eq!(fresh.change(0x7F00_0000), Some(false), "a faint shadow is not the shell's");
    assert_eq!(fresh.change(0x8000_0000), Some(true), "half opaque is");
}

#[test]
fn pointer_shape_told_once_a_change() {
    let mut s = PointerShape { at: Some((10, 10)), said: Some(true) };
    assert_eq!(s.change(0xFF00_0000), None, "the arrow already said");
    s.said = Some(false);
    assert_eq!(s.change(0x0000_0000), None, "already withdrawn");
    assert_eq!(s.change(0xFF00_0000), Some(true));
}

#[test]
fn pointer_shape_codes_are_the_compositors() {
    assert!(CLIENT.contains("const OP: u16 = 0x0018;"));
    assert!(
        CLIENT.contains("const ARROW: u32 = 0;")
            && SHAPES.contains("pub const ARROW_CODE: u8 = 0;")
    );
    assert!(CLIENT.contains("const WITHDRAW: u32 = 0xFF;"));
    assert!(DECODE.contains("pub const CURSOR_WITHDRAW: u32 = 0xFF;"));
}

const RUN: &str = include_str!("../../capsule_desktop_shell/src/server/runner/run.rs");
const POINTER: &str = include_str!("../../capsule_desktop_shell/src/server/input/pointer.rs");

/// Every pass of the serve loop syncs the shape, so a menu opened or closed
/// under a pointer at rest changes it; every motion records the pointer.
#[test]
fn pointer_shape_follows_the_pointer_and_the_frames() {
    assert!(RUN.contains("crate::server::pointer_shape::sync(&mut ctx);"));
    assert!(POINTER.contains("ctx.pointer_shape.at = Some((x as u32, y as u32));"));
}
