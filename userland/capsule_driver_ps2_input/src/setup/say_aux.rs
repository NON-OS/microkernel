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

//! One console line for the aux port, so a photograph of the boot log says
//! whether a PS/2 touchpad or mouse came up and, if not, at which step.

use super::marker::marker;

/// No aux port in the device list, or its interrupt line not bound.
pub(super) fn say_no_aux() {
    marker(b"[driver_ps2] aux port: none listed or no line, PS/2 pointer off\n");
}

/// What `enable_mouse` answered.
pub(super) fn say_aux(outcome: Result<bool, &'static str>) {
    match outcome {
        Ok(true) => marker(b"[driver_ps2] aux port: pointer up, wheel\n"),
        Ok(false) => marker(b"[driver_ps2] aux port: pointer up, no wheel\n"),
        Err(why) => say_off(why.as_bytes()),
    }
}

/// How many bytes the bring-up took for the other port's and dropped
/// (init/read_port.rs): above zero, a held key or a reporting touchpad was
/// kept out of a reply it would have been read as.
pub(super) fn say_dropped(n: u32) {
    const HEAD: &[u8] = b"[driver_ps2] bring-up: bytes from the other port dropped: ";
    let mut line = [0u8; 80];
    line[..HEAD.len()].copy_from_slice(HEAD);
    let mut end = HEAD.len();
    let mut digits = [0u8; 10];
    let (mut v, mut k) = (n, 0);
    loop {
        digits[k] = b'0' + (v % 10) as u8;
        (v, k) = (v / 10, k + 1);
        if v == 0 {
            break;
        }
    }
    for i in (0..k).rev() {
        line[end] = digits[i];
        end += 1;
    }
    line[end] = b'\n';
    let _ = nonos_libc::mk_debug(line.as_ptr(), end + 1);
}

/// The refusal in one write, built on the stack (the proofs build has no
/// allocator), so it reaches the log as one line.
fn say_off(why: &[u8]) {
    const HEAD: &[u8] = b"[driver_ps2] aux port: pointer off, ";
    let mut line = [0u8; 128];
    let why = &why[..why.len().min(line.len() - HEAD.len() - 1)];
    let end = HEAD.len() + why.len();
    line[..HEAD.len()].copy_from_slice(HEAD);
    line[HEAD.len()..end].copy_from_slice(why);
    line[end] = b'\n';
    let _ = nonos_libc::mk_debug(line.as_ptr(), end + 1);
}
