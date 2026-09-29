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

use alloc::string::String;
use alloc::vec::Vec;

use spin::Mutex;

use super::shape::{parse, Shape};

/* Parsed gradients per source string, failures included, oldest dropped
 * first: a value is parsed once, not on every repaint. */
const MAX_SHAPES: usize = 256;
/* Rendered boxes keyed by (source, width, height), most recently used
 * first, holding at most 4 MiB of pixels in all. A gradient is a pure
 * function of those three, so an entry never goes stale. */
pub(super) const MAX_RASTER_BYTES: usize = 4 << 20;

pub(super) type Raster = (String, i32, i32, Vec<u32>);

pub(super) struct Cache {
    pub shapes: Vec<(String, Option<Shape>)>,
    pub rasters: Vec<Raster>,
}

pub(super) static CACHE: Mutex<Cache> =
    Mutex::new(Cache { shapes: Vec::new(), rasters: Vec::new() });

/* The parsed gradient for `src`, parsed and remembered on a miss. */
pub(super) fn shape<'a>(
    shapes: &'a mut Vec<(String, Option<Shape>)>,
    src: &str,
) -> Option<&'a Shape> {
    let i = match shapes.iter().position(|(s, _)| s == src) {
        Some(i) => i,
        None => {
            if shapes.len() >= MAX_SHAPES {
                shapes.remove(0);
            }
            shapes.push((src.into(), parse(src)));
            shapes.len() - 1
        }
    };
    shapes[i].1.as_ref()
}
