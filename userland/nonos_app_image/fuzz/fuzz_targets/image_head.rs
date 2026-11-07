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

//! The image head parser on bytes nobody vouched for. It never panics;
//! whatever it accepts lies inside the total it names, its members tile the
//! data from the names to the end, every name is a package name named once,
//! the listed member is first and alone, and a parse of the head alone
//! agrees with a parse of the whole.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_app_image::{head_len, Head, Role, HEADER_LEN};

fuzz_target!(|data: &[u8]| {
    let Ok(h) = Head::parse(data) else { return };
    let n = head_len(&data[..HEADER_LEN]);
    assert!(n.is_ok(), "an accepted head has a length");
    let n = n.unwrap_or(usize::MAX);
    assert!(n <= data.len());
    let alone = Head::parse(&data[..n]).map(|a| a.members);
    assert_eq!(alone, Ok(h.members.clone()), "the head alone is accepted, and reads the same");
    let mut at = n as u64;
    for (i, m) in h.members.iter().enumerate() {
        assert_eq!(m.at.start, at);
        assert!(m.at.end <= h.total);
        assert_eq!(m.role == Role::Listed, i == 0);
        assert!(h.members[..i].iter().all(|o| o.name != m.name));
        at = m.at.end;
    }
    assert_eq!(at, h.total);
});
