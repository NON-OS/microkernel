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
//! FUNDED's reply as the store takes it: one byte 0 or 1 is the bit; a
//! meter error, no answer or a reply that does not read is no bit at all.

use nonos_payment_proto::{EAGAIN, EINVAL, EPERM, FUNDED_NO, FUNDED_YES};

use crate::store::channel_held::answer;

#[test]
fn one_byte_zero_or_one_is_the_bit() {
    assert_eq!(answer(Ok(Vec::from([FUNDED_YES]))), Some(true));
    assert_eq!(answer(Ok(Vec::from([FUNDED_NO]))), Some(false));
}

#[test]
fn a_meter_error_or_a_reply_that_does_not_read_is_no_bit() {
    for e in [EAGAIN, EPERM, EINVAL] {
        assert_eq!(answer(Err(e)), None, "{e}");
    }
    for bad in [&[][..], &[2], &[1, 0], &[0, 1], &[0xff]] {
        assert_eq!(answer(Ok(bad.to_vec())), None, "{bad:?}");
    }
}
