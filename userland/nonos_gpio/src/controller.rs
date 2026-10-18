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

//! What every controller model offers the router: arm a pin with its
//! trigger, mask and unmask it, list the pins whose interrupt is pending
//! and clear them, read a pin's level and drive an output pin. A pin the
//! model cannot place is refused, never guessed.

use crate::trigger::Trigger;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioError {
    /// The pin is not one this controller can place.
    Unplaced,
    /// The trigger asks for what the pad cannot do.
    Trigger,
    /// The pin is not configured as an output.
    NotOutput,
}

pub trait Controller {
    fn arm(&mut self, pin: u16, trigger: Trigger) -> Result<(), GpioError>;
    fn mask(&mut self, pin: u16, masked: bool) -> Result<(), GpioError>;
    /// Pending pins among `armed`, cleared as they are read.
    fn take_pending(&mut self, armed: &[u16]) -> Vec<u16>;
    fn level(&mut self, pin: u16) -> Result<bool, GpioError>;
    fn drive(&mut self, pin: u16, high: bool) -> Result<(), GpioError>;
}
