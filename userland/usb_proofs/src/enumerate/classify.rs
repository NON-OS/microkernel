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

//! A device below a hub that binds nothing, as a keyboard-less device does:
//! the driver gives its slot back.

use super::devices::Devices;
use super::types::Outcome;
use crate::hub::Place;

pub(super) fn classify(_xhci_port: u32, _slot: u8, _place: Place, _devs: &mut Devices) -> Outcome {
    Outcome::NotHid
}
