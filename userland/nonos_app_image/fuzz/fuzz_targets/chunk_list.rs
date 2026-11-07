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

//! The chunk list parser on bytes nobody vouched for. It never panics; an
//! accepted list's chunks tile the image it names, a list is taken by its
//! own BLAKE3 and refused by any other pin, and no chunk outside the list
//! is ever held.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_app_image::{covering, ChunkList};

fuzz_target!(|data: &[u8]| {
    let pin = *blake3::hash(data).as_bytes();
    let Ok(c) = ChunkList::parse(data, &pin) else {
        assert!(ChunkList::read(data).is_err());
        return;
    };
    let mut other = pin;
    other[31] ^= 1;
    assert!(ChunkList::parse(data, &other).is_err());
    let mut end = 0;
    for i in 0..c.count() {
        let r = c.range(i);
        assert!(r.is_some(), "a counted chunk has a range");
        let r = r.unwrap_or(0..0);
        assert_eq!(r.start, end);
        assert!(r.end > r.start && r.end <= c.total);
        end = r.end;
    }
    assert_eq!(end, c.total);
    assert_eq!(covering(&c, &(0..c.total)).len(), c.count());
    assert!(!c.holds(c.count(), &[]));
});
