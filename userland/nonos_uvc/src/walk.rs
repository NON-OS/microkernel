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
//! A camera in a configuration: its video control interface (class 0Eh,
//! subclass 01h), and its video streaming interface (subclass 02h) with
//! the formats its class descriptors give, and each alternate setting with
//! an isochronous IN endpoint (frames.rs reads the alternates).

use crate::desc::Camera;
use crate::frames::alternate;
use crate::vs::{format, frame};

const DT_INTERFACE: u8 = 0x04;
const CS_INTERFACE: u8 = 0x24;
const CLASS_VIDEO: u8 = 0x0E;
const VIDEO_CONTROL: u8 = 0x01;
const VIDEO_STREAMING: u8 = 0x02;

/// The first camera in configuration `raw`, whose wTotalLength the caller
/// checked, or None without a control and a streaming interface.
pub fn camera(raw: &[u8]) -> Option<Camera> {
    let (mut cam, mut have) = (Camera::default(), (false, false));
    let mut i = raw.first().map_or(0, |&l| l as usize).max(2);
    let mut current: Option<(u8, u8)> = None;
    while i + 2 <= raw.len() {
        let len = raw[i] as usize;
        let d = raw.get(i..i + len).filter(|_| len >= 2)?;
        match d[1] {
            DT_INTERFACE if len >= 9 && d[5] == CLASS_VIDEO => {
                current = Some((d[6], d[2]));
                match (d[6], have) {
                    (VIDEO_CONTROL, (false, _)) => (cam.control, have.0) = (d[2], true),
                    (VIDEO_STREAMING, (_, false)) => (cam.streaming, have.1) = (d[2], true),
                    _ => {}
                }
            }
            DT_INTERFACE => current = None,
            CS_INTERFACE if current == Some((VIDEO_STREAMING, cam.streaming)) => {
                if let Some(f) = format(d) {
                    cam.formats.push(f);
                } else if let (Some(fr), Some(f)) = (frame(d), cam.formats.last_mut()) {
                    f.frames.push(fr);
                }
            }
            _ => {}
        }
        i += len;
    }
    if !(have.0 && have.1) || cam.formats.is_empty() {
        return None;
    }
    cam.alts = alternate(raw, cam.streaming);
    Some(cam)
}
