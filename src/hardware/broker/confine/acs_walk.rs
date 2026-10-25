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

//! The PCIe ACS walk, built for x86_64 only, gated once here rather than on
//! each of its files. The files stay beside the rest of `confine`, which is
//! where `kernel_proofs` mounts them by path, and they name each other as
//! siblings, as they did when `confine` declared them itself.

use super::member;

#[path = "acs.rs"]
pub(super) mod acs;
#[path = "acs_alias.rs"]
mod acs_alias;
#[path = "acs_kind.rs"]
mod acs_kind;
#[path = "acs_layout.rs"]
mod acs_layout;
#[path = "acs_path.rs"]
mod acs_path;
#[path = "acs_reach.rs"]
mod acs_reach;
#[path = "acs_read.rs"]
mod acs_read;
#[path = "acs_rule.rs"]
mod acs_rule;
#[path = "acs_tree.rs"]
mod acs_tree;
#[path = "acs_words.rs"]
mod acs_words;
