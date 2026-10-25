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

//! The capsule's shutdown wording and the kernel's acceptance and choice
//! rules, mounted from their sources; helpers for `shutdown_say_tests`; and
//! the two method lines, for each way a method went.

#[path = "../../capsule_acpi/src/sleep/shutdown_say.rs"]
pub mod say;
#[allow(dead_code)]
#[path = "../../../src/hardware/broker/acpi/sleep_request.rs"]
pub mod sleep_request;

use say::{method_line, Part, Went};

pub fn text(write: impl FnOnce(&mut dyn FnMut(Part))) -> String {
    let mut s = String::new();
    write(&mut |p| match p {
        Part::Str(b) => s.push_str(core::str::from_utf8(b).unwrap()),
        Part::Hex(v) => s.push_str(&format!("{v:#x}")),
        Part::Dec(v) => s.push_str(&v.to_string()),
    });
    s
}

pub fn pairs() -> Vec<Option<(u8, u8)>> {
    let mut v = vec![None];
    v.extend((0..=9).flat_map(|a| (0..=9).map(move |b| Some((a, b)))));
    v
}

#[test]
fn each_method_line_says_how_it_went() {
    let went = [Went::Absent, Went::Ran, Went::Faulted(0x1F3)];
    for (tts, pts) in went.iter().flat_map(|t| went.iter().map(move |p| (*t, *p))) {
        let lines =
            (text(|o| method_line(b"_TTS", tts, o)), text(|o| method_line(b"_PTS", pts, o)));
        let want = |w: &str, g: Went| match g {
            Went::Absent => format!("{w} absent"),
            Went::Ran => format!("{w}(5) ran"),
            Went::Faulted(at) => format!("{w}(5) faulted at {at:#x}"),
        };
        assert_eq!(lines, (want("_TTS", tts), want("_PTS", pts)));
    }
}
