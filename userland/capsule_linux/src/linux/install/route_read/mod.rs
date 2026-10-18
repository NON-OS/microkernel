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

//! Reading a mirror's reply off a stream through the network the person
//! chose. Through the Nym mixnet or the Anyone network the first byte of an
//! answer is many seconds out and the rest can come in pieces seconds apart,
//! so the reader measures quiet in time, not in empty reads: it waits up to
//! FIRST_MS for anything, then up to QUIET_MS after each piece. It stops once
//! `done` says the reply is whole, or the far end has finished. Pure, so
//! capsule_linux_proofs drives it with a scripted stream.

mod offline;
pub mod read;
mod stream;

pub use offline::offline;
pub use read::read_reply;
pub use stream::{Reply, Source};
