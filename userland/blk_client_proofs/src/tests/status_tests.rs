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

//! What virtio-blk now answers, in the words a failed install shows. The
//! driver ends a request it gave up on with E_TIMEDOUT, told apart from
//! E_IO; every status a request can end with is named, not shown as a
//! bare number. The words are the client's (`status_text.rs`), whichever
//! unit last wrote them: none is pinned here.

use crate::drivers::virtio_blk::errno::{E_ACCES, E_INVAL, E_IO, E_MSGSIZE, E_NXIO, E_TIMEDOUT};
use crate::status_text::describe;

#[test]
fn a_request_virtio_blk_gave_up_on_answers_the_errno_the_other_drivers_send() {
    assert_eq!(E_TIMEDOUT, -110);
}

#[test]
fn every_status_a_virtio_blk_request_ends_with_is_named_in_words() {
    for status in [E_ACCES, E_IO, E_NXIO, E_INVAL, E_MSGSIZE, E_TIMEDOUT] {
        let words = describe(status);
        assert_ne!(words, format!("status {status}"), "{status} has no words");
    }
    assert_ne!(describe(E_TIMEDOUT), describe(E_IO), "a timeout reads as a media error");
}
