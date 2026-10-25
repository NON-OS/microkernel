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
//! The probe and commit controls: the bytes sent, and an answer read back,
//! UVC 1.0's 26 bytes included.

use nonos_uvc::{Probe, CONTROL_LEN_11};

#[test]
fn a_probe_is_laid_out_and_read_back() {
    let p = Probe { hint: 1, format: 2, frame: 1, interval: 333_333, max_frame: 0, max_payload: 0 };
    let b = p.bytes();
    assert_eq!(b.len(), CONTROL_LEN_11);
    assert_eq!(&b[..8], &[1, 0, 2, 1, 0x15, 0x16, 0x05, 0]);
    let mut answer = b.to_vec();
    answer[18..22].copy_from_slice(&614_400u32.to_le_bytes());
    answer[22..26].copy_from_slice(&3072u32.to_le_bytes());
    let back = Probe::read(&answer[..26]).expect("UVC 1.0's 26 bytes");
    assert_eq!((back.format, back.frame, back.max_frame, back.max_payload), (2, 1, 614_400, 3072));
    assert_eq!(Probe::read(&answer[..25]), None);
}
