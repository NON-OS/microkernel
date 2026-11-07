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

//! The ring and doorbell of `nonos_surface`, over loom's atomics.

#[cfg(test)]
mod ring_sync {
    pub(crate) use loom::sync::atomic::{AtomicU32, AtomicU64, Ordering};
}

#[cfg(test)]
#[path = "../../nonos_surface/src/ring_error.rs"]
mod ring_error;

#[cfg(test)]
#[path = "../../nonos_surface/src/ring_header.rs"]
mod ring_header;

#[cfg(test)]
#[path = "../../nonos_surface/src/ring.rs"]
mod ring;

#[cfg(test)]
#[path = "../../nonos_surface/src/doorbell.rs"]
mod doorbell;

#[cfg(test)]
mod ring_tests;

#[cfg(test)]
mod model_tests;
