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

//! What the transport knows about the PCI function: its identity, its BARs
//! as the broker listed them, and a snapshot of its config space.

mod bars;
mod config;
mod ids;

pub use bars::{has_io_bar, BarInfo, BarKind, Bars};
pub use config::{ConfigSpace, CONFIG_SPACE_LEN};
pub use ids::{is_modern_id, MODERN_ID_FIRST, MODERN_ID_LAST, VIRTIO_VENDOR_ID};
