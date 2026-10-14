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

mod enable;
mod init;
mod ioapic_rte;
mod irte;
mod msi;
mod route;
mod slots;
mod table;
mod write;

pub use init::init;
pub use ioapic_rte::{handle_of, remappable};
pub use irte::{destination, encode, is_present, source, vector, Irte, Route};
pub use msi::{handle, remapped, MsiMessage};
pub use route::{release_msi, route_entry, route_msi};
pub use slots::{give, take, Slots, SLOT_WORDS};
pub use table::is_remapping;
