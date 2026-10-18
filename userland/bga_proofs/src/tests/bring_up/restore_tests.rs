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

//! What the driver leaves: the DISPI state firmware left, written back
//! while the registers are still mapped, before either mapping or the
//! claim is given up, when the driver goes (and when an attempt fails after
//! the mode is set: `not_taken_tests`). The shim records the registers at each give-back and
//! poisons the register mapping from its unmap, so a write-back that came
//! after it, or after the release, fails here by name.

use nonos_libc::{given_back, untouched_since_unmap};

use super::adapter::{adapter, found, Adapter, MODE_BYTES, REG_LEN};
use super::attempt::run;
use super::left::{showing, state, text_mode, written_back};

/// The calls the drop makes, in order: both mappings, the claim. The
/// primaries' memory is the platform's, given back as it drops.
const GIVEN_BACK: [&str; 3] = ["mmio_unmap", "mmio_unmap", "device_release"];

fn calls_since(before: usize) -> Vec<&'static str> {
    given_back()[before..].iter().map(|g| g.0).collect()
}

#[test]
fn the_driver_going_writes_back_the_state_firmware_left() {
    for left in [text_mode, |a: &Adapter| showing(a, 800, 600)] {
        let a = adapter(REG_LEN, MODE_BYTES);
        left(&a);
        let firmware = state(&a);
        let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("refused: {e:?}"));
        let before = given_back().len();
        drop(d);
        written_back(&a, before, firmware);
        assert!(untouched_since_unmap(), "the register mapping was written after its unmap");
        assert_eq!(calls_since(before), GIVEN_BACK);
    }
}
