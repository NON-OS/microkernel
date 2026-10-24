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
//! Every controller of this kind on the machine served as one host: an
//! Intel ICH has up to six UHCI functions, AMD and other chipsets several
//! OHCI. Their root ports are numbered on, controller after controller,
//! and slots are the host's: a slot is bound to the controller whose port
//! it is addressed on, and every later request for it goes there. Shared
//! by driver.uhci0 and driver.ohci0.

mod address;
pub mod front;
mod give_back;
mod hc;
mod numbering;
mod park;
mod ports;
mod serve;
mod slots;

pub use front::{Front, MAX_HC};
pub use hc::Hc;
pub use serve::serve;
