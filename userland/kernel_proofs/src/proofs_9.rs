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

//! Host proofs, group 9 of 9: test modules mounted by path
//! from proofs.rs, each at its own file.

/// Which process owns which address space, read without the paging lock.
#[path = "asid_directory/mod.rs"]
pub mod asid_directory;
/// MkLocalSign and MkLocalVerify hash an image a bounded chunk at a time.
#[path = "image_digest/mod.rs"]
pub mod image_digest;
/// A guest's peer calls hold its own lock, not one over every guest.
#[path = "peer_stripe/mod.rs"]
pub mod peer_stripe;
/// The paging manager's lock, handed over in the order it was asked for.
#[path = "ticket_lock/mod.rs"]
pub mod ticket_lock;
