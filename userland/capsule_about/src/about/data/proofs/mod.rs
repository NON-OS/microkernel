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

//! The live proof board: the session's attestation and its anonymity route,
//! read from the kernel and the attest service each time the screen is drawn.
//!
//! This window holds AttestRead and nothing that touches the network. The
//! route comes from the attest service's board, where only the transports may
//! post; the window reads it and draws conclusions, it never asks a transport
//! anything itself.

pub mod census;
mod read;
pub mod session;
pub mod words;

pub use read::{read, Snapshot};
