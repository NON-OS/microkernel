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
//! claim is given up, when the driver goes and when an attempt fails after
//! the mode is set. The shim records the registers at each give-back and
//! poisons the register mapping from its unmap, so a write-back that came
//! after it, or after the release, fails here by name.

use nonos_libc::{given_back, refuse_surfaces, untouched_since_unmap};

use super::left::{showing, state, text_mode, written_back};
use crate::adapter::{adapter, found, Adapter, MODE_BYTES, REG_LEN};
use crate::error::BgaError;
use crate::setup::run;

/// The calls a drop makes, in order: the surface's memory, both mappings,
/// the claim. A driver that made its surface releases its share first.
const GIVEN_BACK: [&str; 4] = ["munmap", "mmio_unmap", "mmio_unmap", "device_release"];

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
        assert_eq!(calls_since(before), [&["surface_release"][..], &GIVEN_BACK].concat());
    }
}

#[test]
fn an_attempt_refused_after_the_mode_is_set_writes_the_firmware_state_back() {
    let a = adapter(REG_LEN, MODE_BYTES);
    text_mode(&a);
    let firmware = state(&a);
    refuse_surfaces(true);
    assert_eq!(run(found(MODE_BYTES)).err(), Some(BgaError::SurfaceRefused));
    written_back(&a, 0, firmware);
    assert!(untouched_since_unmap(), "the register mapping was written after its unmap");
    assert_eq!(calls_since(0), GIVEN_BACK);
}
