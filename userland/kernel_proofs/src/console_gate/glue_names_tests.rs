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

//! Each step of the gate is reached by its own name. mod.rs hands them out
//! unrenamed, no `use` anywhere in src/ gives one another name, and the
//! finder of renames is shown to find one.

use super::text::code;
use super::text_items::flat;
use super::text_uses::{every_file, renames};

const STEPS: [&str; 11] = [
    "stopped",
    "drawable",
    "Drawing",
    "wait_quiet",
    "stop_drawing",
    "set_gone",
    "end_takeover",
    "set_held",
    "surrendered",
    "taken",
    "framebuffer_state",
];

#[test]
fn the_gate_is_handed_out_under_its_own_names() {
    let m = flat(&code("kernel_core/init/framebuffer/mod.rs"));
    let held = "useheld::{end_takeover,set_gone,set_held,stop_drawing,surrendered,taken};";
    for line in ["usedrawing::{drawable,wait_quiet};", held] {
        assert_eq!(m.matches(&format!("pub(crate){line}")).count(), 1, "{line}");
    }
    assert_eq!(m.matches("usedrawing::").count() + m.matches("useheld::").count(), 2);
}

#[test]
fn no_use_in_src_gives_a_step_of_the_gate_another_name() {
    let mut seen = 0;
    for (rel, code) in every_file() {
        for (from, to) in renames(&code) {
            seen += 1;
            let step = STEPS.iter().find(|s| **s == from || **s == to);
            assert!(step.is_none(), "{rel}: `{from} as {to}`");
        }
    }
    assert!(seen > 0, "no rename found anywhere: the finder is blind");
}

#[test]
fn the_rename_finder_reads_a_use_and_skips_a_cast() {
    let code = "pub(crate) use held::{set_gone,\n    end_takeover as set_gone};\n\
                let n = x as u32;\nfn f() { because as well }";
    assert_eq!(renames(code), [("end_takeover".to_string(), "set_gone".to_string())]);
}
