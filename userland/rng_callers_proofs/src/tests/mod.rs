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

//! The proofs reached through each caller's public calls. The volume's and
//! the capability engine's sit inside their modules (`volume_tests`,
//! `engine_tests`). Each test sets the RNG for its own thread first.

mod aslr;
mod core_api;
mod cryptofs;
mod cryptofs_files;
mod cryptofs_rotate;
mod kat;
mod keys;
mod keys_rotate;
mod quantum;
mod ramfs;
mod ramfs_files;
mod sanitize;
mod seal;
mod session_account;
mod session_api;
mod session_manager;
mod session_token;
