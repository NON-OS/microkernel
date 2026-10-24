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
//! Codecs on a link, answering the CORB from a description.
//!
//! `model::corb_engine` echoes one fixed word to every command, enough to
//! prove the ring but not a codec walk. Here a device thread reads each
//! command the driver posts, answers it the way the described codec would
//! (parameters, connection lists, pin configurations, coefficients, the jack)
//! and writes the answer into the RIRB with the answering codec's address in
//! the upper word, as a controller does. A command to an address with no
//! codec gets no answer at all, which is what the link does. Every command is
//! kept in order so a test can read back exactly what the driver sent.

mod answer;
mod codec;
mod get_verbs;
mod handle;
mod noise;
mod start;

pub use codec::*;
pub use handle::*;
pub use start::*;
