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

use uefi::table::boot::BootServices;

use crate::entropy::{efi_rng_word, rdrand64};
use crate::security::{audit, init_canaries, AuditEvent};

/// The canary seed: the CPU's generator, else the firmware's (which an
/// aarch64 core without RNDR has), else the counter.
pub fn init_security_primitives(bs: &BootServices) {
    let entropy = rdrand64()
        .or_else(|| efi_rng_word(bs))
        .unwrap_or_else(|| crate::arch::cpu::cycles().rotate_left(17) ^ 0x9E37_79B9_7F4A_7C15);
    init_canaries(entropy);
    audit(AuditEvent::BootStart, 0, b"security init");
}
