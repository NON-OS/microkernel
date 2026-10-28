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

use super::loader::legacy;
use super::loader::tlv::{parse, Version};
use super::rig::{tlv, TYP};

#[test]
fn a_tlv_version_keeps_what_the_download_needs() {
    let v = parse(&tlv(TYP, 0x01, [1, 2, 3], 1)).unwrap();
    assert_eq!((v.cnvi_top, v.cnvr_top, v.variant(), v.platform()), (0x410, 0x410, 0x17, 0x37));
    assert_eq!((v.img_type, v.build, v.sbe_type), (0x01, [1, 2, 3], 1));
    assert!(v.fw_id.is_empty());
}

#[test]
fn a_running_build_is_its_time_stamp_and_number_and_min_fw_comes_last() {
    let running = parse(&[0x1d, 2, 0x1d, 0x1a, 0x1f, 4, 0x18, 0, 0, 0]).unwrap();
    assert_eq!(running.build, [0x18, 0x1d, 0x1a]);
    let min = parse(&[0x1f, 4, 0x18, 0, 0, 0, 0x2d, 3, 1, 2, 3]).unwrap();
    assert_eq!(min.build, [1, 2, 3]);
}

#[test]
fn a_tlv_past_the_end_a_stray_octet_or_a_short_value_is_refused() {
    let mut t = tlv(TYP, 0x01, [0; 3], 0);
    assert!(parse(&t[..t.len() - 1]).is_none());
    t.push(0x99);
    assert!(parse(&t).is_none());
    assert!(parse(&[0x10, 2, 0x10, 0x04]).is_none(), "a CNVi top of two octets");
    assert_eq!(parse(&[0x99, 0]), Some(Version::default()), "an unknown type is passed over");
}

#[test]
fn a_firmware_id_must_be_a_file_name() {
    assert_eq!(parse(&[0x50, 5, b'a', b'b', b'-', b'1', 0]).unwrap().fw_id, b"ab-1");
    assert!(parse(&[0x50, 2, b'a', b'/']).is_none());
}

#[test]
fn the_old_form_is_nine_octets_after_the_status_platform_0x37_first() {
    let l = legacy::parse(&[0x37, 0x13, 0x20, 0x06, 0x01, 0x53, 0x07, 0x1a, 0]).unwrap();
    assert_eq!((l.variant, l.hw_revision, l.fw_variant, l.fw_revision), (0x13, 0x20, 0x06, 1));
    assert_eq!(l.build, [0x53, 0x07, 0x1a]);
    assert!(legacy::parse(&[0x36, 0x13, 0x20, 0x06, 0x01, 0x53, 0x07, 0x1a, 0]).is_none());
    assert!(legacy::parse(&[0x37, 0x13, 0x20, 0x06, 0x01, 0x53, 0x07, 0x1a]).is_none());
    assert!(legacy::parse(&tlv(TYP, 0x01, [0; 3], 0)).is_none(), "TLV is not the old form");
}
