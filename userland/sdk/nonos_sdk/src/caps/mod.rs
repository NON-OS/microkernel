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

//! What an app is allowed to do, declared by the app itself.
//!
//! Every capability an app declares is written in its own source, into the
//! `.nonos.caps` section `sdk_main!` emits. The manifest it is signed and
//! proved against carries the word typed into its `Capsule.mk`, and
//! `scripts/check_declared_caps.py` compares the two after signing. The
//! kernel enforces the manifest's word at spawn. There is no set of powers an app gets for free beyond
//! `BASE`, and nothing it can acquire later.

mod base;
mod groups;

pub use base::BASE;
pub use groups::{BUILD_TOOLING, CRYPTO, DEBUG, IPC, NETWORK, SERVICE, STORAGE, WINDOW};
