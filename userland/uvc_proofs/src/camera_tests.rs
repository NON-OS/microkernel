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
//! A camera found in its configuration: interfaces, formats, frame sizes
//! and the alternate settings with an endpoint, and the one a stream of a
//! given payload takes.

use nonos_uvc::{camera, pick_alt, FormatKind};

use crate::webcam::config;

#[test]
fn a_webcam_is_read_whole() {
    let cam = camera(&config()).expect("a camera");
    assert_eq!((cam.control, cam.streaming), (0, 1));
    let kinds: Vec<FormatKind> = cam.formats.iter().map(|f| f.kind).collect();
    let yuy2 = FormatKind::Uncompressed { fourcc: *b"YUY2", bits: 16 };
    assert_eq!(kinds, vec![yuy2, FormatKind::Mjpeg]);
    let sizes: Vec<(u8, u16, u16, u32)> =
        cam.formats[0].frames.iter().map(|f| (f.index, f.width, f.height, f.max_bytes)).collect();
    assert_eq!(sizes, vec![(1, 640, 480, 614_400), (2, 320, 240, 153_600)]);
    assert_eq!(
        (cam.formats[1].frames[0].width, cam.formats[1].frames[0].interval),
        (1280, 333_333)
    );
    let alts: Vec<(u8, u8, u32)> =
        cam.alts.iter().map(|a| (a.setting, a.endpoint, a.per_interval)).collect();
    assert_eq!(alts, vec![(1, 0x81, 512), (2, 0x81, 3072)], "bits 12:11 count three");
}

#[test]
fn not_a_camera_or_cut_short_is_none() {
    let mut c = config();
    c[14] = 0x03;
    assert_eq!(camera(&c), None, "a HID interface where the control one was");
    let full = config();
    assert_eq!(camera(&full[..40]), None, "streaming interface missing");
    let mut bad = config();
    bad[9] = 200;
    assert_eq!(camera(&bad), None, "a descriptor running past the end");
}

#[test]
fn a_stream_takes_the_smallest_alternate_with_room() {
    let cam = camera(&config()).unwrap();
    assert_eq!(pick_alt(&cam.alts, 400).map(|a| a.setting), Some(1));
    assert_eq!(pick_alt(&cam.alts, 512).map(|a| a.setting), Some(1));
    assert_eq!(pick_alt(&cam.alts, 2000).map(|a| a.setting), Some(2));
    assert_eq!(pick_alt(&cam.alts, 4000), None, "no alternate carries it");
}
