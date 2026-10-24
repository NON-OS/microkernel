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

//! The `BOX` entry against the report it carries: capsule_linux's own kind
//! list, preset names and report version, read from its sources, must match
//! nsh_wire's order, so a record can never misread a count.

use nsh_wire::record::BOX_KINDS;

const BOXED: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_linux/src/linux/boxed/");

fn source(file: &str) -> String {
    std::fs::read_to_string(format!("{BOXED}{file}")).expect("capsule_linux's box sources")
}

/// The quoted words of the first line of `text` that opens with `start`.
fn words_after(text: &str, start: &str) -> Vec<String> {
    let at = text.find(start).expect("the line is there") + start.len();
    let line = &text[at..text[at..].find(']').map(|e| at + e).expect("closed")];
    line.split('"').skip(1).step_by(2).map(str::to_owned).collect()
}

#[test]
fn the_kinds_are_capsule_linux_s_in_its_order() {
    let kinds = words_after(&source("kind.rs"), "[\"write\"");
    let mut want = vec!["write".to_owned()];
    want.extend(kinds);
    assert_eq!(want, BOX_KINDS, "nsh_wire's order is the report's");
    assert!(source("kind.rs").contains("pub const KINDS: usize = 9;"));
}

#[test]
fn the_presets_and_the_version_are_the_report_s() {
    let report = source("report.rs");
    assert!(report.contains("pub const VERSION: u32 = 1;"), "BOX REPORT v1");
    let order = ["Preset::Strict => \"strict\"", "Preset::Offline => \"offline\""];
    let (a, b) = (report.find(order[0]).unwrap(), report.find(order[1]).unwrap());
    assert!(a < b && report.contains("Preset::Allow => \"allow\""));
    // The counts saturate, as the entry's total does.
    assert!(source("tally.rs").contains("saturating_add"));
}
