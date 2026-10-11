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

//! driver.sdhci0's IDENTIFY body as the kernel client reads it.

use super::identity::decode_identity;
use crate::model::{CardCfg, HostCfg, Rig};
use crate::protocol::{encode_identify, MEDIUM_EMMC};

#[test]
fn the_kernel_reads_the_part_the_capsule_names() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let disk = rig.disk(true).expect("bring-up");
    let n = disk.names();
    let body = encode_identify(n.model(), &n.serial, MEDIUM_EMMC).expect("printable");
    let id = decode_identity(&body).expect("the kernel takes it");
    assert_eq!(id.model(), b"SanDisk DF4032");
    assert_eq!(id.serial(), b"12345678");
    assert_eq!(id.medium, 1);
}

#[test]
fn trailing_spaces_go_and_a_name_that_cannot_be_shown_is_not_sent() {
    let body = encode_identify(b"Toshiba 032G  ", b"0000ABCD", MEDIUM_EMMC).expect("taken");
    assert_eq!(decode_identity(&body).expect("valid").model(), b"Toshiba 032G");
    assert!(encode_identify(b"bad\x01name", b"1", 1).is_none());
    assert!(encode_identify(b"x", b"\xff", 1).is_none());
    assert!(encode_identify(&[b'M'; 41], b"1", 1).is_none());
    assert!(encode_identify(b"x", &[b'1'; 21], 1).is_none());
    let blank = encode_identify(b"   ", b"", 1).expect("an empty name is a name");
    assert!(decode_identity(&blank).expect("valid").model().is_empty());
}
