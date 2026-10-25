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

//! One line through the scrub, as text: None for a dropped line.

use nonos_hwreport::redact::{scrub, Scrubbed};
use nonos_hwreport::sink::Sink;

pub fn run(line: &str) -> Option<String> {
    let mut buf = [0u8; 512];
    let mut out = Sink::new(&mut buf);
    match scrub(line.as_bytes(), 235, &mut out) {
        Scrubbed::Dropped => None,
        Scrubbed::Kept => Some(String::from_utf8(out.written().to_vec()).unwrap()),
    }
}
