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

//! Profile lookup by index and by name.

use crate::term::theme::profiles::{by_index, by_name, ABYSS, COUNT, DARK};

/// The preferences record stores the theme as a bare `u16` read off disk, so
/// a corrupt or newer file can hand `by_index` any value at all. The lookup
/// must be total — saturating to DARK, never panicking.
#[test]
fn index_lookup_is_total_and_stable() {
    assert_eq!(COUNT, 4, "COUNT is {}", COUNT);
    assert_eq!(by_index(0).bg, DARK.bg);
    assert_eq!(by_index(3).bg, ABYSS.bg);
    assert_eq!(
        by_index(99).bg,
        DARK.bg,
        "out-of-range index gave {:#010X}, want DARK {:#010X}",
        by_index(99).bg,
        DARK.bg
    );
}

/// Nine profiles collapsed to four. The retired names stay resolvable so an
/// existing `theme blackarch` in muscle memory or a script keeps working
/// instead of erroring out.
#[test]
fn names_resolve_including_retired_ones() {
    for (n, want) in [
        (&b"dark"[..], 0u16),
        (&b"dim"[..], 1),
        (&b"light"[..], 2),
        (&b"abyss"[..], 3),
        (&b"blackarch"[..], 0),
        (&b"black"[..], 0),
        (&b"blue"[..], 0),
        (&b"matrix"[..], 0),
        (&b"glass"[..], 1),
        (&b"smoke"[..], 1),
        (&b"clear"[..], 1),
    ] {
        let got = by_name(n);
        assert_eq!(
            got,
            Some(want),
            "by_name({:?}) = {:?}, want Some({})",
            core::str::from_utf8(n).unwrap(),
            got,
            want
        );
    }
    assert_eq!(by_name(b"nope"), None);
}
