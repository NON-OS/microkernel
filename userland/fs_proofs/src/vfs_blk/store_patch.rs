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

//! The table-of-contents patch, from the shipping file.
//!
//! This is the write that makes a wallet survive a reboot, and it lands in the
//! index of everything the machine persists. An off-by-one would point one
//! record's digest at another record's bytes, and every entry after it would
//! fail its check on the next boot with nothing to say why.

pub use super::error;

#[path = "../../../capsule_vfs/src/blk/store_header.rs"]
pub mod store_header;
#[path = "../../../capsule_vfs/src/blk/store_patch.rs"]
pub mod patch;
#[path = "../../../capsule_vfs/src/blk/store_rules.rs"]
pub mod store_rules;
#[path = "../../../capsule_vfs/src/blk/store_free.rs"]
pub mod store_free;
#[path = "../../../capsule_vfs/src/blk/store_toc.rs"]
pub mod store_toc;
#[path = "../../../capsule_vfs/src/blk/wire.rs"]
pub mod wire;

pub use patch::{entry_sector, patch_digest, patch_entry, DIGEST_AT, DIGEST_LEN, OFFSET_AT};
pub use store_free::free_extent;
pub use store_rules::{in_capsule_tree, permitted, CAPSULE_TREE};
