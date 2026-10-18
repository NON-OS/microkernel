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

//! CTA-861 extensions: detailed timings, short video and audio descriptors.

use crate::build::{base, dtd, dtd_1080p};
use crate::build_ext::{cta, data, with};
use crate::edid::Edid;
use crate::source::Source;

fn tv() -> Vec<u8> {
    let mut blocks = data(2, &[128 + 16, 4, 97, 0, 250]);
    // One L-PCM 2 channel 32/44.1/48 kHz 16/20/24 bit, then a reserved 0.
    blocks.extend(data(1, &[0x09, 0x07, 0x07, 0x00, 0x00, 0x00]));
    let uhd = dtd(594_000, [3840, 560, 176, 88, 2160, 90, 8, 10], (1210, 680), 0x1E);
    with(base(), &[cta(&blocks, &[dtd_1080p(), uhd])])
}

#[test]
fn a_tv_lists_its_dtds_then_its_vics() {
    let raw = tv();
    let e = Edid::parse(&raw).unwrap();
    let mut v = Vec::new();
    e.for_each_timing(&mut |t| v.push(t));
    let got: Vec<_> = v.iter().map(|t| (t.source, t.width, t.height, t.native)).collect();
    assert_eq!(
        got,
        [
            (Source::CtaDetailed, 1920, 1080, false),
            (Source::CtaDetailed, 3840, 2160, false),
            (Source::CtaVic, 1920, 1080, true),
            (Source::CtaVic, 1280, 720, false),
            (Source::CtaVic, 3840, 2160, false),
        ],
        "VIC 250 is not in the table and names nothing"
    );
    assert_eq!(e.preferred().unwrap().h_active, 1920, "the first CTA DTD stands in");
}

#[test]
fn audio_descriptors_come_out_whole_and_basic_audio_is_seen() {
    let raw = tv();
    let e = Edid::parse(&raw).unwrap();
    let mut v = Vec::new();
    e.for_each_audio(&mut |a| v.push(a));
    assert_eq!(v.len(), 1);
    assert_eq!((v[0].format, v[0].channels, v[0].rates, v[0].detail), (1, 2, 7, 7));
    assert!(e.basic_audio());
    assert!(!Edid::parse(&base()).unwrap().basic_audio());
}

#[test]
fn a_bad_extension_checksum_skips_that_block_only() {
    let mut raw = tv();
    raw[128 + 50] ^= 0xFF;
    let e = Edid::parse(&raw).unwrap();
    assert_eq!(e.extension_count(), 1);
    assert_eq!(e.extensions().count(), 0);
    assert!(e.preferred().is_none());
}
