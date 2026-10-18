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

use super::message::answer::answer;
use super::message::encode::encode;
use super::message::tag::{MessageError, Tag, WORDS};

const GET_CLOCK_RATE: u32 = 0x0003_0002;
const ANSWERED: u32 = 0x8000_0000;

fn clock() -> [Tag<'static>; 1] {
    [Tag { id: GET_CLOCK_RATE, values: &[0xc], room: 2 }]
}

#[test]
fn a_clock_request_is_laid_out_as_documented() {
    let mut buf = [0xdead_beef; WORDS];
    // Size, code; tag id, value bytes, tag code, two value words; end tag.
    assert_eq!(encode(&clock(), &mut buf), Ok(32));
    assert_eq!(&buf[..8], &[32, 0, GET_CLOCK_RATE, 8, 0, 0xc, 0, 0]);
}

#[test]
fn the_firmware_answer_is_read_back_and_checked() {
    let tags = clock();
    let mut buf = [0; WORDS];
    encode(&tags, &mut buf).unwrap();
    // The firmware fills the values in and sets both answered bits.
    buf[1] = ANSWERED;
    buf[4] = ANSWERED | 8;
    buf[5..7].copy_from_slice(&[0xc, 250_000_000]);
    assert_eq!(answer(&tags, &buf, 0), Ok(&[0xc, 250_000_000][..]));
    // A length that is not a whole word still covers its last bytes.
    buf[4] = ANSWERED | 5;
    assert_eq!(answer(&tags, &buf, 0), Ok(&[0xc, 250_000_000][..]));
    // Parse error, a tag left unanswered, an answer longer than its room,
    // and a tag the message never asked.
    let refused = |mut b: [u32; WORDS], at: usize, value: u32| {
        b[at] = value;
        answer(&tags, &b, 0).map(|a| a.to_vec())
    };
    assert_eq!(refused(buf, 1, ANSWERED | 1), Err(MessageError::NotAnswered));
    assert_eq!(refused(buf, 4, 8), Err(MessageError::TagNotAnswered));
    assert_eq!(refused(buf, 4, ANSWERED | 12), Err(MessageError::TagNotAnswered));
    assert_eq!(answer(&tags, &buf, 1), Err(MessageError::TagNotAnswered));
}
