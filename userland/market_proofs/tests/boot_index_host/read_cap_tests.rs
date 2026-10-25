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

//! The operator's catalogue is read one byte past 8 MiB and refused past it, never cut.

use crate::clients::vfs;
use crate::fixture::{load_file, said, MAX, NO_BASELINE, PATH};
use crate::ingest;
use crate::process::MARKET_PID;

const PAST_MAX: &[u8] = b"[MARKET] the operator's catalogue was refused: it is past 8 MiB\n";

#[test]
fn a_catalogue_of_exactly_max_bytes_is_read_whole_and_handed_on() {
    let (file, store) = load_file(MAX, None);
    let blobs = ingest::blobs();
    assert_eq!(blobs.len(), 1);
    assert_eq!(blobs[0].len(), MAX);
    assert!(blobs[0] == file);
    assert_eq!(store.installs(), [(1, true, Vec::new())]);
    assert_eq!(said(), [NO_BASELINE]);
}

#[test]
fn a_catalogue_one_byte_past_max_is_refused_never_taken() {
    let (_, store) = load_file(MAX + 1, None);
    assert_eq!(said(), [NO_BASELINE, PAST_MAX]);
    assert_eq!(ingest::blobs().len(), 0);
    assert!(store.installs().is_empty());
}

#[test]
fn the_read_asks_for_one_byte_past_max() {
    load_file(MAX + 1, None);
    assert_eq!(vfs::reads(), [(MARKET_PID, PATH.to_vec(), MAX as u32 + 1)]);
}
