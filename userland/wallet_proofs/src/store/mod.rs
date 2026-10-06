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

//! The keyring's key store from capsule source, file by file as
//! `capsule_keyring/src/store/mod.rs` names them, under the `crate::store`
//! path its own visibility is written against. `eth_secret.rs` is left out:
//! it reads the kernel clock, and nothing proved here goes through it.

#[path = "../../../capsule_keyring/src/store/count.rs"]
mod count;
#[path = "../../../capsule_keyring/src/store/delete.rs"]
mod delete;
#[path = "../../../capsule_keyring/src/store/ended.rs"]
mod ended;
#[path = "../../../capsule_keyring/src/store/eth_valid.rs"]
mod eth_valid;
#[path = "../../../capsule_keyring/src/store/lock.rs"]
mod lock;
#[path = "../../../capsule_keyring/src/store/metadata.rs"]
mod metadata;
#[path = "../../../capsule_keyring/src/store/retrieve.rs"]
mod retrieve;
// The real code's own style, allowed on the include rather than restyled here.
#[allow(clippy::new_without_default)]
#[path = "../../../capsule_keyring/src/store/state.rs"]
mod state;
#[path = "../../../capsule_keyring/src/store/store_key.rs"]
mod store_key;
#[path = "../../../capsule_keyring/src/store/types/mod.rs"]
mod types;
#[path = "../../../capsule_keyring/src/store/unlock.rs"]
mod unlock;
#[path = "../../../capsule_keyring/src/store/wipe.rs"]
mod wipe;

pub use eth_valid::eth_secret_valid;
pub use types::{KeyMetadata, KeyType, Store, StoreError, MAX_KEYS, MAX_KEYS_PER_OWNER};
