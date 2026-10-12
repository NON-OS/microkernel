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
//! A video streaming interface's formats and their frame sizes, from the
//! class-specific VS descriptors (CS_INTERFACE 24h): VS_FORMAT_UNCOMPRESSED
//! (04h), VS_FRAME_UNCOMPRESSED (05h), VS_FORMAT_MJPEG (06h) and
//! VS_FRAME_MJPEG (07h).

use alloc::vec::Vec;

use crate::alt::Alt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatKind {
    /// Uncompressed, with the GUID's first four bytes (a FourCC: YUY2,
    /// NV12) and the bits a pixel takes.
    Uncompressed {
        fourcc: [u8; 4],
        bits: u8,
    },
    Mjpeg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameSize {
    pub index: u8,
    pub width: u16,
    pub height: u16,
    /// The most bytes one frame takes.
    pub max_bytes: u32,
    /// The default interval, in 100 ns units (333333 for 30 fps).
    pub interval: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Format {
    pub index: u8,
    pub kind: FormatKind,
    pub frames: Vec<FrameSize>,
}

/// A camera: its control and streaming interfaces, the streaming one's
/// formats, and its alternate settings with an endpoint.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Camera {
    pub control: u8,
    pub streaming: u8,
    pub formats: Vec<Format>,
    pub alts: Vec<Alt>,
}
