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

//! Setup's answers. Setup keeps two files (`nonos_policy_proto::setup_record`):
//! the answers, then a marker, and writes the marker only once the answers
//! reached the store. On a boot with no store the answers still reach the
//! vfs and the marker does not. The answers are carried whenever they
//! decode, with the marker beside them: on the new disk they are kept,
//! which is what the marker says.

use alloc::vec::Vec;

use nonos_policy_proto::setup_record::{Answers, ANSWERS_PATH, DONE, DONE_PATH};

use super::source::CarrySource;
use crate::store::StoreBuilder;

/// Add the answers and the marker to `store`, and say what they were.
pub(super) fn carry_answers(
    src: &mut dyn CarrySource,
    store: &mut StoreBuilder,
) -> Option<Answers> {
    let answers_path = core::str::from_utf8(ANSWERS_PATH).ok()?;
    let done_path = core::str::from_utf8(DONE_PATH).ok()?;
    let raw: Vec<u8> = src.read(answers_path)?;
    let answers = Answers::decode(&raw)?;
    store.add_all(&[(answers_path, &raw), (done_path, &DONE)]).ok()?;
    Some(answers)
}
