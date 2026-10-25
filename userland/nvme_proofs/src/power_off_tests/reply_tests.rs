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

//! The power-off's reply, and who may ask for it: the kernel alone.

use crate::error::NvmeError;
use crate::medium_rule::allows;
use crate::protocol::{E_IO, E_TIMEDOUT, OP_FLUSH, OP_POWER_OFF};
use crate::server::power_off_rule::status;

#[test]
fn the_reply_names_how_the_shutdown_ended() {
    assert_eq!(status(Ok(())), 0);
    assert_eq!(status(Err(NvmeError::ShutdownTimeout)), E_TIMEDOUT);
    assert_eq!(status(Err(NvmeError::ControllerFatal)), E_IO);
}

#[test]
fn only_the_kernel_may_power_the_disk_off() {
    assert!(allows(OP_POWER_OFF, 0, false));
    assert!(!allows(OP_POWER_OFF, 7, false));
    assert!(!allows(OP_POWER_OFF, 7, true), "a store writer may not shut it down");
    assert!(allows(OP_FLUSH, 7, true), "while it may still flush");
}
