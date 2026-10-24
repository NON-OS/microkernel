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
//! What the capsule running the walk does with a device below a hub: the
//! hub class addresses it, the capsule decides what it is.

use alloc::vec::Vec;

use crate::host::Host;
use crate::node::Hub;
use crate::place::Place;

pub trait Classes<H: Host> {
    /// The device addressed in `slot` at `place`: bound to a class, or
    /// brought up with `attach` and pushed onto `hubs`. True when
    /// something is bound on it; a hub pushed counts as kept too.
    fn classify(&mut self, host: &mut H, slot: u8, place: Place, hubs: &mut Vec<Hub>) -> bool;
    /// A kept slot whose device is gone: drop what is bound on it.
    fn forget(&mut self, slot: u8);
}
