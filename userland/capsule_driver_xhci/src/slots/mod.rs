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
mod admit;
mod asker;
mod claims;
mod dci;
mod owners;
mod pairs;
mod release;
mod resources;
mod sharing;
mod table;
mod uses;
pub use admit::interrupt_limit;
pub use asker::Asker;
pub use dci::dci_from_ep_address;
pub use pairs::{pair_of, pair_stream};
pub use release::Release;
pub use resources::{InterruptEndpoint, SlotResources};
pub use sharing::Sharing;
pub use table::{SlotTable, PORT_FREE};
pub use uses::use_of;
