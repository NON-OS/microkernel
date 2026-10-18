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
//! The capture gate: no stream and no listing without CameraCapture, asked
//! at open and again at every start; one peer's stream at a time, read
//! and stopped by that peer alone.

use crate::gate::{Session, CAMERA_CAPTURE};
use crate::wire::{E_BUSY, E_PERM};

#[test]
fn without_camera_capture_nothing_opens_or_starts() {
    let mut s = Session::default();
    assert_eq!(s.open(false), Err(E_PERM));
    assert_eq!(s.start(7, false), Err(E_PERM));
    assert_eq!(s.streamer, None, "nothing streams");
    assert_eq!(CAMERA_CAPTURE, 137_438_953_472, "bit 37");
}

#[test]
fn one_peer_streams_and_only_it_reads_or_stops() {
    let mut s = Session::default();
    assert_eq!((s.open(true), s.start(7, true)), (Ok(()), Ok(())));
    assert_eq!(s.start(8, true), Err(E_BUSY), "another peer's stream runs");
    assert_eq!((s.own(7), s.own(8)), (Ok(()), Err(E_PERM)));
    assert_eq!(s.start(7, false), Err(E_PERM), "asked again at every start");
    s.stop();
    assert_eq!((s.own(7), s.start(8, true)), (Err(E_PERM), Ok(())));
}
