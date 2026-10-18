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

//! An attempt whose mode the adapter does not take (`clamp`): the read-back
//! refuses it as ModeNotTaken after `set_mode` wrote every mode register,
//! and its drop writes the firmware's state back as any other does, before
//! the register mapping is given up and with nothing written after.

use nonos_libc::untouched_since_unmap;

use super::adapter::{adapter, found, MODE_BYTES, REG_LEN};
use super::attempt::run;
use super::clamp::clamping;
use super::left::{state, text_mode, written_back};
use crate::error::BgaError;

#[test]
fn an_attempt_refused_as_mode_not_taken_writes_the_firmware_state_back() {
    let a = adapter(REG_LEN, MODE_BYTES);
    text_mode(&a);
    let firmware = state(&a);
    clamping(&a);
    assert_eq!(run(found(MODE_BYTES)).err(), Some(BgaError::ModeNotTaken));
    written_back(&a, 0, firmware);
    assert!(untouched_since_unmap(), "the register mapping was written after its unmap");
}
