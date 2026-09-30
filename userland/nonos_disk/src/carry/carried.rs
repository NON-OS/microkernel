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

//! What was carried, and what was not, for the screen that says so.

use nonos_policy_proto::setup_record::Answers;

use crate::store::StoreImage;

pub struct Carried {
    /// The new disk's store, ready to write.
    pub store: StoreImage,
    /// Setup's answers, when this boot holds them.
    pub answers: Option<Answers>,
    /// Signed programs carried, each as its four files.
    pub programs: usize,
    /// Signed programs past what vfs loads, and their bytes.
    pub left_out: usize,
    pub left_out_bytes: u64,
    /// Signed programs not carried for another reason: a file the vfs
    /// listed and would not return, or a path the store's table cannot hold.
    pub skipped: usize,
}
