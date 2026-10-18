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

//! Firmware's Debug stores and Fatal, said on serial.

use crate::sys::log::Line;
use nonos_aml::object::Object;

pub(super) fn debug(value: &Object) {
    let line = Line::new(b"debug ");
    match value {
        Object::Integer(v) => line.hex(*v).end(),
        Object::String(s) => line.str(&s.borrow()).end(),
        _ => line.str(b"(object)").end(),
    }
}

pub(super) fn fatal(kind: u8, code: u32, arg: u64) {
    let l = Line::new(b"Fatal type=").hex(kind as u64).str(b" code=").hex(code as u64);
    l.str(b" arg=").hex(arg).end();
}
