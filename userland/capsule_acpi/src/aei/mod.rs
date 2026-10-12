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

//! GPIO_EVENT (13): the ACPI events a GPIO controller's pins signal
//! (_AEI), listed for the controller's driver to arm, and run (_Exx, _Lxx
//! or _EVT) when it reports one. `rate` is pure.

mod controllers;
mod list;
mod rate;
mod run;
mod wire;

pub use controllers::GpioEvents;
pub use wire::gpio_event;
