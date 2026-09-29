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

/* Firmware that answers GetVariable with the specification's attribute words and
 * error codes. Every other service answers EFI_UNSUPPORTED. */

use crate::arch::x86_64::uefi::types::Guid;

const EFI_SUCCESS: u64 = 0;
const EFI_BUFFER_TOO_SMALL: u64 = 0x8000_0000_0000_0005;

fn name_of(ptr: *const u16) -> String {
    let mut out = String::new();
    let mut i = 0;
    loop {
        let c = unsafe { *ptr.add(i) };
        if c == 0 {
            return out;
        }
        out.push(c as u8 as char);
        i += 1;
    }
}

fn firmware_word(name: &str) -> u32 {
    match name {
        "SecureBoot" | "SetupMode" => 0x06,
        _ => 0x27,
    }
}

pub(super) extern "efiapi" fn get_variable(
    name: *const u16,
    _guid: *const Guid,
    attributes: *mut u32,
    size: *mut u64,
    data: *mut u8,
) -> u64 {
    unsafe {
        *attributes = firmware_word(&name_of(name));
        if data.is_null() {
            *size = 1;
            return EFI_BUFFER_TOO_SMALL;
        }
        *data = 1;
        *size = 1;
    }
    EFI_SUCCESS
}
