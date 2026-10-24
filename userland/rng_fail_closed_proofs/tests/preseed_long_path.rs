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

//! A caller's path too long for the line is cut short; the line still ends
//! with its line number, and is no longer than its buffer.

#![allow(dead_code, clippy::let_unit_value)]

include!("../src/mount.rs");

#[path = "long/a_source_path_long_enough_to_be_cut_short"]
mod long {
    #[path = "on_the_serial_line_so_the_line_stays"]
    pub mod deeper {
        #[path = "within_its_buffer_and_still_names_its_line/caller.rs"]
        pub mod caller;
    }
}

#[test]
fn a_long_path_is_cut_short() {
    stand_in::sources(false, true, false);
    let (file, at) = long::deeper::caller::draw();
    let said = String::from_utf8(stand_in::SERIAL.lock().unwrap().clone()).unwrap();
    let head = "[RNG] pre-seed draw by ";
    assert!(head.len() + file.len() > 148, "the path is long enough to cut");
    let kept = &file[..148 - head.len()];
    assert_eq!(said, format!("{head}{kept}:{at}\n"));
}
