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

//! The lines for a controller not brought up and a bring-up step that failed.

use crate::controller::ControllerInfo;
use crate::discover::Found;
use crate::error::{reason, NvmeError};
use crate::log::{emit, Line};

pub fn attempt_failed(dev: &Found, e: NvmeError) {
    let mut line = Line::new();
    line.text(b"controller ")
        .hex_digits(dev.vendor as u64, 4)
        .text(b":")
        .hex_digits(dev.device as u64, 4)
        .text(b" not brought up: ")
        .text(reason(e).as_bytes());
    emit(&mut line);
}

pub fn unsupported(info: &ControllerInfo, mapped: u64) {
    let mut line = Line::new();
    line.text(b"not an NVMe register block or doorbells past BAR0: CAP ")
        .hex(info.cap, 16)
        .text(b" VS ")
        .hex(info.version as u64, 8)
        .text(b" CSTS ")
        .hex(info.csts as u64, 8)
        .text(b" mapped ")
        .dec(mapped);
    emit(&mut line);
}

pub fn step_failed(step: &[u8], e: NvmeError, info: Option<&ControllerInfo>) {
    let mut line = Line::new();
    line.text(step).text(b": ").text(reason(e).as_bytes());
    if let Some(info) = info {
        line.text(b" (CAP.TO ").dec(info.timeout_units() as u64).text(b" x 500 ms)");
    }
    emit(&mut line);
}
