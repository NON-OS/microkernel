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

//! What linux_inputs.rs checks its readers' answers with: that a slice
//! lies inside what it came from, and a package's gzip members. An .apk is
//! members back to back, from a mirror: each must start where the last
//! ended, together they cover every byte, all of them stay under the
//! bound, and checking a package never panics.

use nonos_capsule_linux_proofs::install::auth::package;

const BOUND: usize = 1 << 20;

pub fn inside(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    inner.is_empty() || (i >= o && i + inner.len() <= o + outer.len())
}

pub fn members(b: &[u8]) {
    if let Ok(parts) = nonos_inflate::members_checked(b, BOUND) {
        let mut at = 0;
        for m in &parts {
            assert!(m.start == at && m.start < m.end && m.end <= b.len(), "a member out of place");
            at = m.end;
        }
        assert_eq!(at, b.len(), "bytes no member covers");
        assert!(parts.iter().map(|m| m.body.len()).sum::<usize>() <= BOUND, "past the bound");
    }
    let _ = package::verified(b, &[0; 20]);
}
