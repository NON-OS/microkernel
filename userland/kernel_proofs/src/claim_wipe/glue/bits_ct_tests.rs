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

//! Device Control 2 as a wipe puts it back (L80): its completion timeout
//! value and disable, which a network holder may write, come back as
//! firmware left them at enumeration, never as the last holder set them,
//! and every other bit as it was read before the reset.

use super::super::bits_rule::{devctl2_after, CT_BITS};

/// Completion Timeout Value (bits 0 to 3) and Disable (bit 4), as recalled;
/// unverified.
const CT: u16 = 0x001F;

#[test]
fn the_timeout_bits_are_the_ones_the_rule_puts_back() {
    assert_eq!(CT_BITS, CT, "the rule's bits are not the completion timeout's");
}

#[test]
fn the_completion_timeout_comes_back_as_firmware_left_it() {
    let mut differ = 0u32;
    for before in 0..=u16::MAX {
        for boot in (0..=u16::MAX).step_by(257) {
            let after = devctl2_after(before, boot);
            assert_eq!(after & CT, boot & CT, "{before:#06x} {boot:#06x}: holder's timeout kept");
            assert_eq!(after & !CT, before & !CT, "{before:#06x} {boot:#06x}: a bit lost");
            differ += u32::from(before & CT != boot & CT);
        }
    }
    assert!(differ > 3_000_000, "too few holders changed the timeout: {differ}");
}
