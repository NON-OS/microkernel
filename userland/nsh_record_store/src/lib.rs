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

//! Durable session records for an nsh/1 host. A `Store` is a directory the
//! host owns; a `Keeper` follows one connection's server events and keeps
//! that session's record and checkpoints there:
//!
//! ```text
//! <session id in hex>.record   the record bytes, in the order they grew
//! <session id in hex>.ckpts    every nsh-ckpt/1, end to end, CKPT_LEN each
//! ```
//!
//! A checkpoint is written only once the record bytes before it are on
//! disk, so a checkpoint never names an entry a crash lost. Files are
//! created owner-only and never reopened: a second session with the same id
//! is refused, not written over.

mod files;
mod keeper;
mod store;

pub use keeper::Keeper;
pub use store::Store;

/// The length of one `nsh-ckpt/1`: magic, version and flags (8), the
/// session id (16), the entry it covers (4), the chain (64), the signature.
pub const CKPT_LEN: usize = 8 + 16 + 4 + 64 + nsh_wire::keys::HYBRID_SIG_LEN;
