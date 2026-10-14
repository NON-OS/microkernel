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

//! Bringing the gen3 radio up on the claimed card, once, before the serving
//! loop starts: check the register window, take the NIC (which makes the RF
//! id readable), choose the firmware by MAC and RF type, map the control,
//! receive and firmware regions as broker-sized grants, boot to ALIVE and run
//! the post-ALIVE commands. Each step that fails names itself (`Failure`) and
//! prints one line; a failure after the device was told where its memory is
//! stops the device and keeps the grants mapped, so nothing the device may
//! still write to is handed back.

mod alive;
mod finish;
mod first;
mod identify;
mod map;
mod report;
mod seen;

pub use first::bring_up;
pub use seen::{Seen, Up};
