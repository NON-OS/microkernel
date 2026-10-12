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
//! A webcam's configuration as one is laid out: a video control
//! interface, then a video streaming interface whose alternate 0 carries
//! the class descriptors (YUY2 at 640x480 and 320x240, MJPEG at
//! 1280x720) and whose alternates 1 and 2 carry an isochronous IN
//! endpoint of 512 bytes and of 3 x 1024 bytes.

fn frame(sub: u8, index: u8, w: u16, h: u16, max: u32) -> Vec<u8> {
    let mut d = vec![30, 0x24, sub, index, 0];
    d.extend(w.to_le_bytes());
    d.extend(h.to_le_bytes());
    d.extend([0u8; 8]);
    d.extend(max.to_le_bytes());
    d.extend(333_333u32.to_le_bytes());
    d.extend([1]);
    d.extend(333_333u32.to_le_bytes());
    d
}

pub fn config() -> Vec<u8> {
    let mut c = vec![9, 2, 0, 0, 2, 1, 0, 0x80, 250];
    c.extend([9, 4, 0, 0, 1, 0x0E, 0x01, 0, 0]);
    c.extend([13, 0x24, 0x01, 0x10, 0x01, 13, 0, 0, 0, 0, 0, 1, 1]);
    c.extend([9, 4, 1, 0, 0, 0x0E, 0x02, 0, 0]);
    c.extend([14, 0x24, 0x01, 2, 0, 0, 0x81, 0, 3, 0, 0, 0, 1, 0]);
    let mut yuy2 = vec![27, 0x24, 0x04, 1, 2];
    yuy2.extend(*b"YUY2\x00\x00\x10\x00\x80\x00\x00\xaa\x00\x38\x9b\x71");
    yuy2.extend([16, 1, 0, 0, 0, 0]);
    c.extend(yuy2);
    c.extend(frame(0x05, 1, 640, 480, 614_400));
    c.extend(frame(0x05, 2, 320, 240, 153_600));
    c.extend([11, 0x24, 0x06, 2, 1, 0, 1, 0, 0, 0, 0]);
    c.extend(frame(0x07, 1, 1280, 720, 1_843_200));
    c.extend([9, 4, 1, 1, 1, 0x0E, 0x02, 0, 0]);
    c.extend([7, 5, 0x81, 0x05, 0x00, 0x02, 1]);
    c.extend([9, 4, 1, 2, 1, 0x0E, 0x02, 0, 0]);
    c.extend([7, 5, 0x81, 0x05, 0x00, 0x14, 1]);
    let total = c.len() as u16;
    c[2..4].copy_from_slice(&total.to_le_bytes());
    c
}
