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

//! The capsule's eleven pins, read from the digests published for
//! linux-firmware tag 20260916: each names its own file and digest.

use super::rtl::pin::Pin;
use super::rtl::pins::*;
use super::rtl::pins_config::*;

const ALL: [&Pin; 11] = [
    &RTL8723B_FW,
    &RTL8723D_FW,
    &RTL8821A_FW,
    &RTL8821C_FW,
    &RTL8761A_FW,
    &RTL8761BU_FW,
    &RTL8822B_FW,
    &RTL8723BS_CONFIG,
    &RTL8821C_CONFIG,
    &RTL8761BU_CONFIG,
    &RTL8822B_CONFIG,
];

#[test]
fn each_real_pin_names_its_own_file_and_digest() {
    for (i, a) in ALL.iter().enumerate() {
        assert!(a.len > 0 && a.file.ends_with(".bin"));
        assert!(a.file.ends_with("_fw.bin") ^ a.file.ends_with("_config.bin"), "{}", a.file);
        for b in &ALL[i + 1..] {
            assert_ne!(a.file, b.file);
            assert_ne!(a.sha256, b.sha256, "{} and {}", a.file, b.file);
        }
    }
    assert_eq!(RTL8821C_FW.sha256[..4], [0xdf, 0xe7, 0xef, 0x77]);
    assert_eq!(RTL8821C_FW.sha256[31], 0x3c);
    assert_eq!(RTL8761BU_CONFIG.len, 6);
}
