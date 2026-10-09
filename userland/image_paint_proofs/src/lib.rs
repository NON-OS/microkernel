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

//! Host proofs for the image decoders and the browser's image and paint
//! path. The toolkit is a dependency; the browser's own sources compile
//! unchanged through #[path] under a module tree at the paths they expect
//! (crate::browser::*), as the capsule builds them.
extern crate alloc;

pub mod browser;
pub mod fixtures;
pub mod shim;

#[cfg(test)]
mod blit_clip_tests;
#[cfg(test)]
mod blit_img;
#[cfg(test)]
mod blit_ref;
#[cfg(test)]
mod blit_tests;
#[cfg(test)]
mod bmp_tests;
#[cfg(test)]
mod datauri_tests;
#[cfg(test)]
mod gif_build;
#[cfg(test)]
mod gif_fixture_tests;
#[cfg(test)]
mod gif_tests;
#[cfg(test)]
mod grad_clip_tests;
#[cfg(test)]
mod grad_fb;
#[cfg(test)]
mod grad_tests;
#[cfg(test)]
mod mask_fade_tests;
#[cfg(test)]
mod object_pos_tests;
#[cfg(test)]
mod png_adam7_tests;
#[cfg(test)]
mod png_build;
#[cfg(test)]
mod png_crc;
#[cfg(test)]
mod png_rows;
#[cfg(test)]
mod png_stream_tests;
#[cfg(test)]
mod png_suite_tests;
#[cfg(test)]
mod stack_tests;
#[cfg(test)]
mod svg_arc;
#[cfg(test)]
mod svg_arc_tests;
#[cfg(test)]
mod svg_size_tests;
