// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::handoff::jump::{
    jump_to_kernel, validate_entry_address, validate_handoff_address, validate_stack_address,
};

pub struct JumpAddresses {
    pub entry: u64,
    pub stack: u64,
    pub handoff: u64,
}

/// Validate addresses and jump to kernel. POST codes on failure: E1=entry, E2=stack, E3=handoff
/// (E4 is `halt_map_overflow`).
pub fn validate_and_jump(addrs: JumpAddresses) -> ! {
    com1_byte(b'V');
    if !validate_entry_address(addrs.entry) {
        com1_byte(b'1');
        halt_with_code(0xE1);
    }
    if !validate_stack_address(addrs.stack) {
        com1_byte(b'2');
        halt_with_code(0xE2);
    }
    if !validate_handoff_address(addrs.handoff) {
        com1_byte(b'3');
        halt_with_code(0xE3);
    }
    com1_byte(b'>');
    com1_byte(b'\n');
    // SAFETY: all addresses validated above
    unsafe { jump_to_kernel(addrs.entry, addrs.stack, addrs.handoff) }
}

/// A memory map that did not fit its buffer: halted rather than handed on
/// cut short. POST code E4.
pub fn halt_map_overflow() -> ! {
    com1_byte(b'4');
    halt_with_code(0xE4)
}

fn com1_byte(b: u8) {
    crate::arch::cpu::post_byte(b)
}

/// Output the POST code (port 0x80 on x86_64) and halt.
fn halt_with_code(code: u8) -> ! {
    crate::arch::cpu::halt_with_code(code)
}
