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

//! The pack parser on bytes nobody vouched for. It never panics; whatever
//! it accepts lies inside the input, names nothing outside its root, keeps
//! no two files over one byte, finds each name it holds and no other, and,
//! when every file is its digest, is exactly what the builder writes back.
//! Its index alone, as a run reads it, is accepted too.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_linux_pack::{build, Index, Kind, Pack, Source, HEADER_LEN};

fuzz_target!(|data: &[u8]| {
    let Ok(p) = Pack::parse(data) else { return };
    // What a run holds of an accepted pack, its index alone, is accepted too.
    let head = Index::data_start(&data[..HEADER_LEN], data.len()).expect("its header");
    assert!(Index::parse(&data[..head], data.len()).is_ok(), "the index of an accepted pack");
    let mut sources = Vec::new();
    let mut whole = true;
    for i in 0..p.len() {
        let e = p.entry(i).expect("an accepted pack's entries read");
        assert!(e.data.end <= data.len());
        assert!(!e.name.starts_with(b"/") && !e.name.split(|b| *b == b'/').any(|c| c == b".."));
        assert_eq!(p.find(e.name).map(|f| f.data), Some(e.data.clone()), "found as itself");
        let mut other = e.name.to_vec();
        other.push(b'~');
        assert!(p.find(&other).is_none_or(|f| f.name == &other[..]));
        let bytes = match e.kind {
            Kind::Dir => &[][..],
            _ => match p.file(&e) {
                Ok(b) => b,
                Err(_) => {
                    whole = false;
                    continue;
                }
            },
        };
        sources.push(Source { name: e.name.to_vec(), kind: e.kind, exec: e.exec, bytes });
    }
    if whole {
        assert_eq!(build(sources).expect("rebuilt"), data, "one pack has one layout");
    }
});
