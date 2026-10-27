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

use super::kept_name::{is_kept, may_reach};
use std::cell::Cell;

#[test]
fn a_homes_slots_the_index_and_the_packs_are_kept_names() {
    for name in ["/h-0123456789abcdef0123456789abcdef.a", "/h-x.b", "/li.a", "/li.b", "/lp-9f"] {
        assert!(is_kept(name.as_bytes()), "{name}");
    }
}

#[test]
fn the_names_on_the_volume_today_are_not() {
    let today = ["/prove.request", "/registry.transcript", "/proof-ab12", "/qwen2.5-0.5b.gguf"];
    for name in today.iter().chain(&["/h", "/hx-1", "/li", "/lib.a", "/lp", "/llama.gguf"]) {
        assert!(!is_kept(name.as_bytes()), "{name}");
    }
}

#[test]
fn a_kept_name_needs_foreign_exec_and_no_other_name_asks() {
    let asked = Cell::new(0);
    let foreign = |answer| {
        asked.set(asked.get() + 1);
        answer
    };
    assert!(!may_reach(b"/h-ab.a", || foreign(false)));
    assert!(may_reach(b"/h-ab.a", || foreign(true)));
    assert_eq!(asked.get(), 2);
    assert!(may_reach(b"/qwen2.5-0.5b.gguf", || foreign(false)));
    assert_eq!(asked.get(), 2, "an ordinary name asked about ForeignExec");
}
