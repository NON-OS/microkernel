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

//! Which ENDs before CONNECTED send the open to another exit, that it goes
//! once only, and the bytes kept to send behind it.

use crate::stream::again::{retries, sends_again};
use crate::stream::held::{keep, Held, EARLY_MAX, WAIT_SECONDS};
use crate::stream::{Stream, StreamStage};

/// EXITPOLICY, TIMEOUT, NOROUTE, HIBERNATING and RESOURCELIMIT.
const RETRIED: [u8; 5] = [4, 7, 8, 9, 11];

#[test]
fn the_exits_own_reasons_retry_and_the_destinations_do_not() {
    for reason in 0..=u8::MAX {
        assert_eq!(retries(reason), RETRIED.contains(&reason), "reason {reason}");
    }
    assert!(!retries(3), "CONNECTREFUSED is the destination's answer");
    assert!(!retries(2), "RESOLVEFAILED is the name's");
}

#[test]
fn an_open_goes_again_once_and_only_while_it_is_held() {
    let mut held = Held::new(b"node.example:18081\0".to_vec(), 18081, 100, false);
    assert_eq!(held.until, 100 + WAIT_SECONDS);
    for reason in RETRIED {
        assert!(sends_again(Some(&held), reason));
    }
    held.again = true;
    for reason in RETRIED {
        assert!(!sends_again(Some(&held), reason), "a second END is passed back");
    }
    assert!(!sends_again(None, 4), "nothing held, nothing to send");
}

#[test]
fn bytes_sent_before_connected_are_kept_to_the_bound() {
    let mut stream = Stream::new(9, 0x8000_0001);
    stream.held = Some(Held::new(b"h:1\0".to_vec(), 1, 0, false));
    keep(&mut stream, &[1u8; 1000]);
    keep(&mut stream, &[2u8; EARLY_MAX - 1000]);
    assert_eq!(stream.held.as_ref().map(|h| h.early.len()), Some(EARLY_MAX));
    keep(&mut stream, &[3u8]);
    assert!(stream.held.is_none(), "one byte past the bound and the open is let go");

    let mut open = Stream::new(10, 0x8000_0001);
    open.held = Some(Held::new(b"h:1\0".to_vec(), 1, 0, true));
    open.stage = StreamStage::Open;
    keep(&mut open, b"later");
    assert_eq!(open.held.map(|h| h.early.len()), Some(0), "nothing kept once connected");
}
