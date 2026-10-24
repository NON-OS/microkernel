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

//! What every Intel display engine generation's code shares: how it reaches
//! the hardware (`access`), why it refuses (`error`), the pipes and
//! transcoders (`pipe`), and the contract a generation's mode setting meets
//! (`modeset`). A leaf: the capsule's engine and each generation's own crate
//! build against it, so neither depends on the other.

#![no_std]

pub mod access;
pub mod error;
pub mod modeset;
pub mod pipe;
