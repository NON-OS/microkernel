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

//! A recording stand-in for firmware's eject methods, any one of which
//! can be made to fail.

use crate::hotplug::Steps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    Lock(bool),
    Undock,
    Eject,
}

pub struct Fake {
    pub calls: Vec<Call>,
    pub fail: Option<Call>,
}

impl Fake {
    fn call(&mut self, c: Call) -> bool {
        self.calls.push(c);
        self.fail != Some(c)
    }
}

impl Steps for Fake {
    fn lock(&mut self, on: bool) -> bool {
        self.call(Call::Lock(on))
    }
    fn undock(&mut self) -> bool {
        self.call(Call::Undock)
    }
    fn eject(&mut self) -> bool {
        self.call(Call::Eject)
    }
}
