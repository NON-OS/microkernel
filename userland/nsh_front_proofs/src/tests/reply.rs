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

//! A reply is read only as far as the kernel wrote it and the buffer holds
//! it, and a payload only if all of it is there.

use crate::reply::{payload, written};

#[test]
fn a_reply_is_what_the_kernel_wrote() {
    let buf = [7u8; 64];
    assert_eq!(written(&buf, 0), Some(&buf[..0]));
    assert_eq!(written(&buf, 20), Some(&buf[..20]));
    assert_eq!(written(&buf, 64), Some(&buf[..]));
}

#[test]
fn a_count_past_the_buffer_or_an_error_is_refused() {
    let buf = [7u8; 64];
    assert_eq!(written(&buf, 65), None);
    assert_eq!(written(&buf, i64::MAX), None);
    assert_eq!(written(&buf, -1), None);
    assert_eq!(written(&buf, i64::MIN), None);
}

#[test]
fn a_payload_must_lie_inside_the_reply() {
    let reply = [1u8; 30];
    assert_eq!(payload(&reply, 20, 10), Some(&reply[20..30]));
    assert_eq!(payload(&reply, 20, 0), Some(&reply[20..20]));
    assert_eq!(payload(&reply, 20, 11), None);
    assert_eq!(payload(&reply, 31, 0), None);
    assert_eq!(payload(&reply, 20, usize::MAX), None, "no wrap past the end");
}
