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

use super::landing::check_bti_landing_pad;
use super::pad::{BTI_C, BTI_J, BTI_JC, PACIASP, PACIBSP};

const NOP: u32 = 0xD503201F;
const BARE_BTI: u32 = 0xD503241F;

fn check(word: u32) -> bool {
    check_bti_landing_pad(&word as *const u32 as u64)
}

#[test]
fn only_real_landing_pads_pass() {
    for pad in [BTI_C, BTI_J, BTI_JC, PACIASP, PACIBSP] {
        assert!(check(pad), "{pad:#x}");
    }
    assert!(!check(NOP));
    assert!(!check(BARE_BTI));
    assert!(!check(0));
}
