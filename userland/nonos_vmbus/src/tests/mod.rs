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

//! The channel layer against a model of the host's side written from
//! QEMU's vmbus.c (`crate::model`): rings both ways, wrapping, full, broken and
//! malformed; the signalling rules; the GPADL, open and close in order, and
//! every step's failure undone.

mod mem_tests;
mod open_tests;
mod ring_tests;
