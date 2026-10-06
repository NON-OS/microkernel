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

// The vfs block layer, from capsule source, every file of it included once
// and under the module names the capsule gives them, so each file's `super::`
// reaches the neighbours it reaches in vfs_pool. The two kernel calls under
// it are nonos_libc's, which on the host is the shim over a disk in memory
// (libc_shim): the boot load, the appender, the remover and the status word
// run here as they do on a machine. The failure taxonomy in `error` is also
// what the vfs handlers turn into errnos.

#[path = "../../../capsule_vfs/src/blk/client.rs"]
mod client;
#[path = "../../../capsule_vfs/src/blk/error.rs"]
pub mod error;
#[path = "../../../capsule_vfs/src/blk/load/mod.rs"]
pub mod load;
#[path = "../../../capsule_vfs/src/blk/patience.rs"]
pub mod patience;
#[path = "../../../capsule_vfs/src/blk/status.rs"]
pub mod status;
#[path = "../../../capsule_vfs/src/blk/streamed.rs"]
pub mod streamed;
#[path = "../../../capsule_vfs/src/blk/store.rs"]
pub mod store;
#[path = "../../../capsule_vfs/src/blk/store_drop.rs"]
pub mod store_drop;
#[path = "../../../capsule_vfs/src/blk/store_entry.rs"]
pub mod store_entry;
#[path = "../../../capsule_vfs/src/blk/store_free.rs"]
pub mod store_free;
#[path = "../../../capsule_vfs/src/blk/store_header.rs"]
pub mod store_header;
#[path = "../../../capsule_vfs/src/blk/store_remove.rs"]
pub mod store_remove;
#[path = "../../../capsule_vfs/src/blk/store_replace.rs"]
pub mod store_replace;
#[path = "../../../capsule_vfs/src/blk/store_room.rs"]
pub mod store_room;
#[path = "../../../capsule_vfs/src/blk/store_rules.rs"]
pub mod store_rules;
#[path = "../../../capsule_vfs/src/blk/store_toc.rs"]
pub mod store_toc;
#[path = "../../../capsule_vfs/src/blk/store_write.rs"]
pub mod store_write;
#[path = "../../../capsule_vfs/src/blk/wire.rs"]
pub mod wire;

/*
 * The capsule's store_patch.rs. The name `store_patch` belongs to the facade
 * below, which carries it and the modules it reads under one path for the
 * table proofs; store_entry's `super::store_patch` finds its items there.
 */
#[path = "../../../capsule_vfs/src/blk/store_patch.rs"]
pub mod patch;

/*
 * The table-of-contents patch that a same-length replacement writes, with the
 * header and toc modules it reads, so the offsets under test are the shipping
 * ones rather than numbers repeated here.
 */
pub mod store_patch;
