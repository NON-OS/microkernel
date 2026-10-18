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

//! The boot line for each device brought up: its name, and whether it
//! points absolutely.

use nonos_libc::mk_debug;

use crate::init::name;
use crate::setup::Input;

pub fn up(input: &Input) {
    let mut line = [0u8; 96];
    let mut n = 0;
    let mut push = |bytes: &[u8]| {
        for &b in bytes.iter().take(line.len() - 1 - n) {
            line[n] = b;
            n += 1;
        }
    };
    push(b"driver.virtio_input: up, ");
    if let Some(cfg) = input.window.device {
        let (name, len) = name(cfg.region);
        push(&name[..len]);
    }
    if input.axes.x.is_some() && input.axes.y.is_some() {
        push(b" (absolute pointer)");
    }
    line[n] = b'\n';
    let _ = mk_debug(line.as_ptr(), n + 1);
}
