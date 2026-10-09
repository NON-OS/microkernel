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

mod atlas;
mod bytes;
mod card;
mod chip;
mod emblem;
mod frame;
mod frame_math;
mod glyph;
mod label;
mod mark;
pub mod palette;
mod particles;
mod parts;
mod shape;
mod scene;
mod style;
mod text;
mod wrap;

pub use atlas::Face;
pub use card::card;
pub use chip::{chip, chip_height, chip_rows, chip_width};
pub use emblem::{draw_captions, draw_emblem};
pub use frame::draw_frame;
pub use label::{label, label_centered, label_width, marker};
pub use mark::{draw_mark, mark_size};
pub use particles::draw_particles;
pub use parts::{dot, draw_tracked, keycap, keycap_width, tracked_width};
pub use shape::round_rect;
pub use style::mark_face;
pub use scene::{scene, Scene};
pub use style::{class, unit, Style};
pub use text::{draw, draw_centered, metrics, width};
pub use wrap::{draw_wrapped, lines};
