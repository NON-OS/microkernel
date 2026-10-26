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
//! capsule_linux's learn files, mounted beside agentproof_policy's own mount
//! of the box, so the file a learn run prints is read by the reader that
//! checks every policy file.

pub use agentproof_policy::boxed::{dests, folders, preset};

#[path = "../../../../capsule_linux/src/linux/boxed/connected.rs"]
pub mod connected;
#[path = "../../../../capsule_linux/src/linux/boxed/learn_folders.rs"]
pub mod learn_folders;
#[path = "../../../../capsule_linux/src/linux/boxed/learned.rs"]
pub mod learned;
#[path = "../../../../capsule_linux/src/linux/boxed/learned_text.rs"]
pub mod learned_text;
