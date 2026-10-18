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

//! A mode the adapter does not take, end to end: an adapter that clamps the
//! width it is given, as one with less video memory than the mode needs
//! does. The read-back fails the attempt by name before the clear, so
//! nothing is drawn into a mode the adapter is not showing. The adapter
//! answers each write as it is made (`clamp`): as a thread it raced the
//! read-back, and a run it lost, retried a hundred times, proved nothing.

use nonos_libc::given_back;

use super::adapter::{adapter, found, DEVICE, MODE_BYTES, REG_LEN};
use super::attempt::run;
use super::clamp::clamping;
use crate::error::BgaError;

#[test]
fn a_mode_the_adapter_clamps_fails_by_name_with_nothing_drawn() {
    let a = adapter(REG_LEN, MODE_BYTES);
    clamping(&a);
    assert_eq!(run(found(MODE_BYTES)).err(), Some(BgaError::ModeNotTaken));
    let drawn = (0..MODE_BYTES).step_by(4).find(|&o| a.fb.wrote32(o) != 0);
    assert_eq!(drawn, None, "drawn into a mode the adapter did not take");
    let last = given_back().last().copied();
    assert_eq!(last, Some(("device_release", DEVICE)), "the claim was kept");
}
