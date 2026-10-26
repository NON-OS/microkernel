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

use super::render::write_line;
use crate::sys::serial::Line;

const TAG_OK: u32 = 0x0000FF00;
const TAG_WARN: u32 = 0x0000AAFF;
const TAG_ERR: u32 = 0x000000FF;
const TAG_INFO: u32 = 0x00FFCC00;
const TAG_STAGE: u32 = 0x00AAAAAA;

/* "[tag] msg" to the serial port as one write under its lock, so another
 * CPU's line never lands between the tag and the message (serial/line.rs). */
fn serial_line(tag: &str, msg: &str) {
    Line::new().str(b"[").str(tag.as_bytes()).str(b"] ").str(msg.as_bytes()).end();
}

pub fn info(msg: &str) {
    serial_line("INFO", msg);
    write_line("INFO", msg, TAG_INFO);
}

pub fn ok(tag: &str, msg: &str) {
    serial_line(tag, msg);
    write_line(tag, msg, TAG_OK);
}

pub fn warn(msg: &str) {
    serial_line("WARN", msg);
    write_line("WARN", msg, TAG_WARN);
}

pub fn error(msg: &str) {
    serial_line("ERROR", msg);
    write_line("ERROR", msg, TAG_ERR);
}

pub fn stage(tag: &str, msg: &str) {
    serial_line(tag, msg);
    write_line(tag, msg, TAG_STAGE);
}
