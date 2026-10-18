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

//! One log line from the parts a pure wording module gives
//! (`sleep::shutdown_say`, `boot::bringup_say`); no line when it gives none.

use super::log::Line;
use crate::sleep::shutdown_say::Part;

pub fn emit(write: impl FnOnce(&mut dyn FnMut(Part))) {
    let mut line: Option<Line> = None;
    write(&mut |p| {
        let l = line.take().unwrap_or_else(|| Line::new(b""));
        line = Some(match p {
            Part::Str(s) => l.str(s),
            Part::Hex(v) => l.hex(v),
            Part::Dec(v) => l.dec(v),
        });
    });
    if let Some(l) = line {
        l.end();
    }
}
