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

use super::window::registers_fit;

const PAGE: usize = 4096;

#[test]
fn registers_past_the_page_are_refused() {
    assert!(registers_fit(0, 0xFF << 8, PAGE));
    assert!(!registers_fit(0, 0x100 << 8, PAGE));
    assert!(registers_fit(0xFF << 24, 0, PAGE));
    assert!(!registers_fit(0x100 << 24, 0, PAGE));
    assert!(!registers_fit((0xF0 << 24) | (0x10 << 40), 0, PAGE));
}

#[test]
fn every_field_value_agrees_with_the_byte_bounds() {
    for iro in 0..1024u64 {
        let fits = iro * 16 + 16 <= PAGE as u64;
        assert_eq!(registers_fit(0, iro << 8, PAGE), fits, "iro {iro}");
    }
    for fro in (0..1024u64).step_by(7) {
        for nfr in 0..256u64 {
            let fits = fro * 16 + (nfr + 1) * 16 <= PAGE as u64;
            assert_eq!(registers_fit((fro << 24) | (nfr << 40), 0, PAGE), fits);
        }
    }
}
