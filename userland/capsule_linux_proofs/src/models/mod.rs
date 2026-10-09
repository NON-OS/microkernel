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

//! The personality's pinned models, assembled for the host.

#[path = "../../../capsule_linux/src/linux/file/models/hex.rs"]
pub mod hex;
#[path = "../../../capsule_linux/src/linux/file/models/pinned.rs"]
pub mod pinned;
#[path = "../../../capsule_linux/src/linux/install/apps.rs"]
pub mod apps;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_qwen25.rs"]
pub mod pinned_qwen25;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_qwen25_big.rs"]
pub mod pinned_qwen25_big;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_qwen3.rs"]
pub mod pinned_qwen3;
#[path = "../../../capsule_linux/src/linux/file/models/pinned_coder.rs"]
pub mod pinned_coder;
#[path = "../../../capsule_linux/src/linux/install/apps_qwen25.rs"]
pub mod apps_qwen25;
#[path = "../../../capsule_linux/src/linux/install/apps_qwen3.rs"]
pub mod apps_qwen3;
#[path = "../../../capsule_linux/src/linux/install/apps_coder.rs"]
pub mod apps_coder;
#[path = "../../../capsule_linux/src/linux/install/chat_pick.rs"]
pub mod chat_pick;
