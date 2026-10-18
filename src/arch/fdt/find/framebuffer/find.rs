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

//! The framebuffer firmware left running and described in the tree as a
//! `simple-framebuffer` node: its memory, its size in pixels, its pitch and
//! its pixel format (devicetree binding `simple-framebuffer`).

use crate::arch::fdt::parser::Fdt;

use super::format::PixelFormat;
use super::region::region;
use super::types::{Refused, SimpleFramebuffer};

const NEEDLES: &[&[u8]] = &[b"simple-framebuffer"];

/// The first enabled `simple-framebuffer`, `None` when the tree names none.
pub fn find(fdt: &Fdt) -> Result<Option<SimpleFramebuffer>, Refused> {
    let Some(found) = fdt.devices(NEEDLES).next() else {
        return Ok(None);
    };
    let (node, _) = found?;
    let (base, size) = region(fdt, &node)?.ok_or(Refused::Missing)?;
    let cell = |name: &[u8]| -> Result<u32, Refused> {
        match fdt.property(&node, name)? {
            Some(p) if p.data.len() == 4 => Ok(p.u32()?),
            _ => Err(Refused::Missing),
        }
    };
    let (width, height, stride) = (cell(b"width")?, cell(b"height")?, cell(b"stride")?);
    let name = fdt.property(&node, b"format")?.ok_or(Refused::Missing)?;
    let format = PixelFormat::parse(name.string()?).ok_or(Refused::UnknownFormat)?;
    let row = u64::from(width) * u64::from(format.bytes_per_pixel());
    let rows = u64::from(stride) * u64::from(height);
    if width == 0 || height == 0 || u64::from(stride) < row || rows > size {
        return Err(Refused::Geometry);
    }
    Ok(Some(SimpleFramebuffer { base, size, width, height, stride, format }))
}
