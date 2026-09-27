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

//! The type roles, each one face, one size, one tracking and one colour, as
//! the phones' Typography and the parts that use it set them. Desktop draws
//! the base sizes; the phones draw the same roles a quarter larger.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::{draw_text_tracked, line_height_with, measure_tracked};

use super::face::{font, Face};
use super::tokens::{TEXT, TEXT_3};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    ScreenLabel,
    Lead,
    RowName,
    RowValue,
    Fact,
    Button,
    ActionLabel,
    Status,
}

struct Spec(Face, f32, f32, bool, u32);

fn spec(role: Role) -> Spec {
    match role {
        Role::ScreenLabel => Spec(Face::Mono, 12.0, 2.4, true, TEXT_3),
        Role::Lead | Role::RowName => Spec(Face::Sans, 14.0, 0.0, false, TEXT_3),
        Role::RowValue => Spec(Face::Mono, 13.0, 0.0, false, TEXT),
        Role::Fact => Spec(Face::Mono, 12.0, 0.0, false, TEXT),
        Role::Button => Spec(Face::Mono, 12.0, 1.6, true, TEXT),
        Role::ActionLabel => Spec(Face::SansMedium, 13.0, 0.0, false, TEXT),
        Role::Status => Spec(Face::Mono, 10.0, 0.6, false, TEXT_3),
    }
}

fn shaped(caps: bool, text: &str) -> String {
    if caps {
        text.to_uppercase()
    } else {
        String::from(text)
    }
}

/// Draw in the role's own colour; returns the pen x after the run.
pub fn draw(fb: &mut PaintBuffer, x: i32, top: i32, role: Role, text: &str) -> i32 {
    let colour = spec(role).4;
    draw_in(fb, x, top, role, text, colour)
}

/// Draw in a given colour, for the few places the design recolours a role:
/// ink on cyan, text-3 on a disabled control.
pub fn draw_in(fb: &mut PaintBuffer, x: i32, top: i32, role: Role, text: &str, argb: u32) -> i32 {
    let Spec(face, px, track, caps, _) = spec(role);
    let Some(f) = font(face) else { return x };
    let (w, h, stride) = (fb.width, fb.height, fb.stride_words as usize);
    draw_text_tracked(f, fb.pixels, stride, w, h, x, top, &shaped(caps, text), argb, px, track)
}

pub fn width(role: Role, text: &str) -> i32 {
    let Spec(face, px, track, caps, _) = spec(role);
    font(face).map_or(0, |f| measure_tracked(f, &shaped(caps, text), px, track))
}

pub fn line(role: Role) -> i32 {
    let Spec(face, px, ..) = spec(role);
    font(face).map_or(0, |f| line_height_with(f, px))
}
