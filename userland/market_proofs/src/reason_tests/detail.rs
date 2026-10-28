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

//! The failed install's line, read as `mk_app_install_detail` answers it.

use nonos_market_proto::install_detail;

const LINE: &[u8] = b"libzmq-4.3.5-r2 (its checksum did not match the signed index)";

#[test]
fn the_line_reads_back_as_the_kernel_filled_it() {
    let mut buf = [0u8; 120];
    buf[..LINE.len()].copy_from_slice(LINE);
    let got = install_detail(&buf, LINE.len() as i64);
    assert_eq!(got, Some("libzmq-4.3.5-r2 (its checksum did not match the signed index)"));
}

#[test]
fn no_line_an_errno_or_a_length_past_the_buffer_reads_as_none() {
    let buf = [b'a'; 120];
    assert_eq!(install_detail(&buf, 0), None);
    assert_eq!(install_detail(&buf, -22), None);
    assert_eq!(install_detail(&buf, 121), None);
    assert_eq!(install_detail(&buf, i64::MAX), None);
}

#[test]
fn a_control_byte_or_non_ascii_is_never_shown() {
    for bad in [&b"pkg\x1b[2J"[..], b"pkg\n(x)", b"caf\xc3\xa9", b"pkg\x7f"] {
        assert_eq!(install_detail(bad, bad.len() as i64), None, "{bad:?}");
    }
}
