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

// Mirrors abi/wire.toml [graphics.surface_descriptor_v1] and
// [input.event_v1]. Padding is fixed by the explicit u64 fields so
// every supported triple lays it out the same way.

pub const SURFACE_FORMAT_ARGB8888: u32 = 1;

pub const INPUT_KIND_KEY_DOWN: u16 = 0;
pub const INPUT_KIND_KEY_UP: u16 = 1;
pub const INPUT_KIND_POINTER_REL: u16 = 2;
pub const INPUT_KIND_POINTER_ABS: u16 = 3;
pub const INPUT_KIND_WHEEL: u16 = 4;
pub const INPUT_KIND_BUTTON_DOWN: u16 = 5;
pub const INPUT_KIND_BUTTON_UP: u16 = 6;
pub const INPUT_KIND_TOUCH: u16 = 7;
/// A gamepad's button: `code` its number from 1, `y` the pad.
pub const INPUT_KIND_PAD_BUTTON_DOWN: u16 = 8;
pub const INPUT_KIND_PAD_BUTTON_UP: u16 = 9;
/// A gamepad's axis: `code` its Generic Desktop usage (30h X to 38h, 39h
/// the hat), `x` its value (-32767..32767; the hat 0 to 7, -1 centred),
/// `y` the pad.
pub const INPUT_KIND_PAD_AXIS: u16 = 10;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SurfaceDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub byte_len: u64,
    pub base_va: u64,
    pub flags: u64,
    /// Written by an attach: the process that registered the surface.
    pub owner_pid: u32,
    /// Written by an attach: 1 when the view is writable.
    pub writable: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct InputEvent {
    pub kind: u16,
    pub flags: u16,
    pub code: u32,
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
    pub timestamp_ns: u64,
}
